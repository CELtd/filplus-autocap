use std::env;
use std::str::FromStr;
use dotenv::dotenv;
use anyhow::{Result, anyhow, Context};
use log::warn;
use serde_json::{json, Value};
use serde::de::DeserializeOwned;
use fvm_shared::address::Address;
use fvm_shared::ActorID;
use fvm_shared::econ::TokenAmount;
use fvm_shared::message::Message;
use fvm_ipld_encoding::{RawBytes, to_vec};
use filecoin_signer::{key_derive, transaction_sign};
use base64::engine::general_purpose;
use base64::Engine as _;
use ethers_core::abi::{Function, Param, ParamType, StateMutability, Token};
use secp256k1::{Message as SecpMessage, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use rlp::RlpStream;
use hex;


use hex::decode;
use reqwest::blocking::Client;
//use ethabi::{Function, Param, ParamType, Token};


use crate::wallet::{Wallet};
use crate::allocation::TransferParams;
use crate::rpc::Connection;
use crate::constants::datacap_actor::{DATACAP_ACTOR_ID, DATACAP_TRANSFER_FUNCTION_ID};
use crate::constants::gas::{SEND_FIL_GAS, ALLOCATION_GAS};

/// Load Lotus devnet JWT token from environment.
fn load_token_from_env() -> Result<String, anyhow::Error> {
    dotenv().ok(); // load from .env
    let token = env::var("LOTUS_JWT")?;
    Ok(format!("Bearer {}", token))
}
// ----------------------------------
// Chain Head + Block Information
// ----------------------------------

/// Fetch the current head block number (epoch).
pub fn get_chain_head_block_number(connection: &Connection) -> Result<u64> {
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.ChainHead",
        "params": [],
        "id": 1
    });

    let response = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?
        .json::<Value>()?;

    let block_number = response["result"]["Height"]
        .as_u64()
        .ok_or_else(|| anyhow!("Missing height in ChainHead response"))?;

    Ok(block_number)
}

/// Fetch full block info including messages at a specific height.
pub fn get_block_info(connection: &Connection, block_number: &u64) -> Result<Value> {
    // Get TipSetKey at height
    let tipset_req = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.ChainGetTipSetByHeight",
        "params": [block_number, []],
        "id": 1
    });

    let tipset_resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .json(&tipset_req)
        .send()?
        .json::<Value>()?;

    let cids = tipset_resp["result"]["Cids"]
        .as_array()
        .ok_or_else(|| anyhow!("Missing CIDs in tipset response"))?;

    // Build valid CID object array: [{"/": "cid"}]
    let cid_array = cids
        .iter()
        .map(|cid| json!({ "/": cid["/"].as_str().unwrap_or("") }))
        .collect::<Vec<_>>();

    // Request all messages in the tipset
    let messages_req = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.ChainGetMessagesInTipset",
        "params": [cid_array],
        "id": 1
    });

    // Get Messages in TipSet
    let messages_resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .json(&messages_req)
        .send()?
        .json::<Value>()?;

    Ok(messages_resp["result"].clone())
}

// ----------------------------------
// Wallet and Address Operations
// ----------------------------------

/// Fetch nonce for next transaction.
pub fn fetch_nonce(connection: &Connection, address: &str) -> Result<u64> {
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.MpoolGetNonce",
        "params": [address.to_string()],
        "id": 1
    });

    let resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?
        .json::<serde_json::Value>()?;

    let nonce = resp["result"].as_u64().ok_or_else(|| anyhow::anyhow!("Nonce missing in response"))?;
    Ok(nonce)
}

/// Get FIL balance of an address (attoFIL).
pub fn fetch_balance(connection: &Connection, address: &str) -> Result<String> {
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.WalletBalance",
        "params": [address.to_string().clone()],
        "id": 1
    });

    let res = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?
        .json::<serde_json::Value>()?;

    Ok(res["result"].as_str().unwrap_or("0").to_string())
}

/// Resolve a Filecoin address to numeric ActorID.
pub fn resolve_id_address(connection: &Connection, address: &str) -> Result<ActorID> {
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.StateLookupID",
        "params": [address, null],
        "id": 1
    });

    let res = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?
        .json::<serde_json::Value>()?;

    let id_str = res["result"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid response: {:?}", res))?;

    if id_str.starts_with("f0") || id_str.starts_with("t0") {
        let num = id_str[2..].parse::<ActorID>()?;
        Ok(num)
    } else {
        anyhow::bail!("Expected ID address, got {}", id_str);
    }
}

// ----------------------------------
// DataCap Queries
// ----------------------------------

/// Get current verified datacap balance of an address.
pub fn fetch_datacap_balance(connection: &Connection, address: &str) -> Result<u64> {
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.StateVerifiedClientStatus",
        "params": [address.to_string().clone(), null],
        "id": 1
    });

    let resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?
        .json::<serde_json::Value>()?;

    let value_str = resp["result"]
        .as_str()
        .unwrap_or("0");

    let balance = u64::from_str(value_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse datacap balance: {}", e))?;
    
    Ok(balance)
}

// ----------------------------------
// Sending Transactions
// ----------------------------------

/// Send FIL from a wallet to another address.
pub fn send_fil_to(connection: &Connection, from: &Wallet, to: &str, amount_atto: &str) -> Result<String> {

    // Step 1: Fetch nonce
    let nonce = fetch_nonce(&connection, &from.address)?;

    // Step 2: Derive key
    let key = key_derive(&from.mnemonic, &from.derivation_path, "", &from.language)?;
    
    let gas_limit = SEND_FIL_GAS;

    // Step 3: Estimate gas premium
    let gas_premium: String = call_rpc(
        connection,
        "Filecoin.GasEstimateGasPremium",
        &serde_json::json!([
            0,
            &from.address,
            gas_limit,
            null
        ]),
    )?;
    let gas_premium_atto = gas_premium.parse::<u128>()?;

    // Step 4: Estimate gas fee cap
    let fee_cap: String = call_rpc(
        connection,
        "Filecoin.GasEstimateFeeCap",
        &serde_json::json!([
            {
                "Version": 0,
                "To": &to,
                "From": &from.address,
                "Nonce": nonce,
                "Value": amount_atto.to_string(),
                "GasLimit": gas_limit,
                "GasFeeCap": "0",
                "GasPremium": gas_premium_atto.to_string(),
                "Method": 0,
                "Params": ""
            },
            0,
            null
        ]),
    )?;
    let gas_fee_cap_atto = fee_cap.parse::<u128>()?;

    // Step 5: Build message
    let message = Message {
        version: 0,
        from: Address::from_str(&from.address.clone())?,
        to: Address::from_str(&to)?,
        sequence: nonce,
        value: TokenAmount::from_atto(amount_atto.parse::<u128>()?),
        method_num: 0,
        params: RawBytes::new(vec![]),
        gas_limit: gas_limit,
        gas_fee_cap: TokenAmount::from_atto(gas_fee_cap_atto),
        gas_premium: TokenAmount::from_atto(gas_premium_atto),
    };

    // Step 6: Sign it
    let signed = transaction_sign(&message, &key.private_key)?;

    // Step 7: Build correct JSON structure manually
    let push_msg = json!({
        "Message": {
            "Version": message.version,
            "To": message.to.to_string(),
            "From": message.from.to_string(),
            "Nonce": message.sequence,
            "Value": message.value.atto().to_string(),
            "GasLimit": message.gas_limit,
            "GasFeeCap": message.gas_fee_cap.atto().to_string(),
            "GasPremium": message.gas_premium.atto().to_string(),
            "Method": message.method_num,
            "Params": general_purpose::STANDARD.encode(message.params.to_vec()),
        },
        "Signature": {
            "Type": signed.signature.signature_type() as u8,
            "Data": general_purpose::STANDARD.encode(signed.signature.bytes()),
        }
    });

    // Step 8: Push the signed message
    let cid_str = push_msg_to_mempool(&connection, &push_msg)?;
    Ok(cid_str)

}

/// Create a datacap allocation transaction from a TransferParams object.
pub fn create_datacap_allocation(transfer_params:TransferParams, connection: &Connection, wallet: &Wallet) -> Result<String> {

    // Step 1: Fetch nonce
    let nonce = fetch_nonce(&connection, &wallet.address)?;
    
    // Step 2: Encode the parameters for the allocation
    let params_vec = to_vec(&transfer_params)?;
    let raw_params = RawBytes::new(params_vec.clone());

    let gas_limit = ALLOCATION_GAS; 

    // Step 3: Estimate gas premium
    let gas_premium: String = call_rpc(
        connection,
        "Filecoin.GasEstimateGasPremium",
        &serde_json::json!([
            0,
            &wallet.address,
            gas_limit,
            null
        ]),
    )?;
    let gas_premium_atto = gas_premium.parse::<u128>()?;

    // Step 4: Estimate gas fee cap
    let fee_cap: String = call_rpc(
        connection,
        "Filecoin.GasEstimateFeeCap",
        &serde_json::json!([
            {
                "Version": 0,
                "To": Address::new_id(DATACAP_ACTOR_ID).to_string(),
                "From": &wallet.address,
                "Nonce": nonce,
                "Value": "0",
                "GasLimit": gas_limit,
                "GasFeeCap": "0",
                "GasPremium": gas_premium_atto.to_string(),
                "Method": 0,
                "Params": general_purpose::STANDARD.encode(&params_vec),
            },
            0,
            null
        ]),
    )?;
    let gas_fee_cap_atto = fee_cap.parse::<u128>()?;


    // Setp 5: Create the message properly encoding the different fields
    let message = Message {
        version: 0,
        from: Address::from_str(&wallet.address)?,
        to: Address::new_id(DATACAP_ACTOR_ID),
        sequence: nonce,
        value: TokenAmount::from_atto(0u8),
        method_num: DATACAP_TRANSFER_FUNCTION_ID,
        params: raw_params,
        gas_limit: ALLOCATION_GAS,
        gas_fee_cap: TokenAmount::from_atto("2000000000".parse::<u128>()?),
        gas_premium: TokenAmount::from_atto("0".parse::<u128>()?),
    };

    // Step 6: Derive key and sign it
    let key = key_derive(&wallet.mnemonic, &wallet.derivation_path, "", &wallet.language)?;
    let signed = transaction_sign(&message, &key.private_key)?;

    // Step 7: Build proper JSON
    let push_msg = json!({
        "Message": {
            "Version": message.version,
            "To": message.to.to_string(),
            "From": message.from.to_string(),
            "Nonce": message.sequence,
            "Value": message.value.atto().to_string(),
            "GasLimit": message.gas_limit,
            "GasFeeCap": message.gas_fee_cap.atto().to_string(),
            "GasPremium": message.gas_premium.atto().to_string(),
            "Method": message.method_num,
            "Params": general_purpose::STANDARD.encode(params_vec),
        },
        "Signature": {
            "Type": signed.signature.signature_type() as u8,
            "Data": general_purpose::STANDARD.encode(signed.signature.bytes()),
        }
    });

    // Step 8: Push message
    let cid = push_msg_to_mempool(&connection, &push_msg)?;
    Ok(cid)

}


pub fn get_chain_id(connection: &Connection) -> Result<u64> {

    let response: serde_json::Value = connection.client
        .post(&connection.rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_chainId",
            "params": []
        }))
        .send()?
        .json()?;

    let hex_chain_id = response["result"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing chainId in response"))?;

    let chain_id = u64::from_str_radix(hex_chain_id.trim_start_matches("0x"), 16)?;
    Ok(chain_id)
}

// Requests minting datacap in our masterbot client wallet from the metallocator contract subtracting allowance from our allcator wallet
pub fn request_datacap(
    connection: &Connection,
    contract_address: &str,
    private_key_hex: &str,
    client_bytes: &[u8],
    datacap: u64,
) -> Result<String> {
    // Setup keys
    let secp = Secp256k1::new();
    let private_key = SecretKey::from_str(private_key_hex.strip_prefix("0x").unwrap_or(private_key_hex))?;
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &private_key);
    let sender = &Keccak256::digest(&public_key.serialize_uncompressed()[1..])[12..]; // 20 bytes

    // Get nonce
    let nonce_resp: serde_json::Value = connection.client
        .post(&connection.rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getTransactionCount",
            "params": [format!("0x{}", hex::encode(sender)), "latest"]
        }))
        .send()?
        .json()?;

    let nonce = u64::from_str_radix(nonce_resp["result"].as_str().unwrap().trim_start_matches("0x"), 16)?;

    // Encode ABI calldata for addVerifiedClient(bytes,uint256)
    let calldata = {
        let func = Function {
            name: "addVerifiedClient".to_string(),
            inputs: vec![
                Param { name: "client".into(), kind: ParamType::Bytes, internal_type: None },
                Param { name: "datacap".into(), kind: ParamType::Uint(256), internal_type: None },
            ],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
            constant: None,
        };
        func.encode_input(&[
            Token::Bytes(client_bytes.to_vec()),
            Token::Uint(datacap.into())
        ])?
    };

    // Gas & fee
    let gas_limit = 30_000_000u64;
    let gas_price = 100_000_000u64;
    let chain_id = get_chain_id(connection)?; // assumes `get_chain_id` returns u64

    // Decode recipient address
    let to = hex::decode(contract_address.strip_prefix("0x").unwrap_or(contract_address))?;

    // Create RLP for signing
    let mut rlp = RlpStream::new_list(9);
    rlp.append(&nonce);
    rlp.append(&gas_price);
    rlp.append(&gas_limit);
    rlp.append(&to);
    rlp.append(&0u8); // value
    rlp.append(&calldata);
    rlp.append(&chain_id);
    rlp.append(&0u8); // r
    rlp.append(&0u8); // s

    // Sign transaction hash
    let hash = Keccak256::digest(&rlp.out());
    let msg = SecpMessage::from_slice(&hash)?;
    let sig = secp.sign_ecdsa_recoverable(&msg, &private_key);
    let (rec_id, sig_bytes) = sig.serialize_compact();
    let v = chain_id * 2 + 35 + rec_id.to_i32() as u64;

    // RLP encode final transaction
    let mut final_rlp = RlpStream::new_list(9);
    final_rlp.append(&nonce);
    final_rlp.append(&gas_price);
    final_rlp.append(&gas_limit);
    final_rlp.append(&to);
    final_rlp.append(&0u8); // value
    final_rlp.append(&calldata);
    final_rlp.append(&v);
    final_rlp.append(&&sig_bytes[0..32]);
    final_rlp.append(&&sig_bytes[32..64]);

    let raw_tx = format!("0x{}", hex::encode(final_rlp.out()));

    // Send the raw transaction
    let res: serde_json::Value = connection.client
        .post(&connection.rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_sendRawTransaction",
            "params": [raw_tx]
        }))
        .send()?
        .json()?;

    Ok(res["result"].as_str().unwrap_or("null").to_string())
}

// ----------------------------------
// Helpers
// ----------------------------------

/// Push a signed message to the Lotus mempool.
pub fn push_msg_to_mempool(connection: &Connection, push_msg: &Value) -> Result<String> {
    // Build the RPC request
    let push_req = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.MpoolPush",
        "params": [push_msg],
        "id": 1
    });

    // Load JWT token for devnet
    let token = load_token_from_env()?;

    // Send the request and parse response
    let push_resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .header("Authorization", token) // Only for devnet
        .json(&push_req)
        .send()?
        .json::<Value>()?;

    // Extract the CID string
    let cid_str = push_resp["result"]["/"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing CID in response"))?
        .to_string();

    Ok(cid_str)
}

/// Check the success of a tx
pub fn check_msg_success(connection: &Connection, cid: &str) -> Result<bool> {

    // Load JWT token for devnet
    let token = load_token_from_env()?;

    let replay_req = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.StateWaitMsg",
        "params": [{"/": cid}, 0],
        "id": 1
    });

    let resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .header("Authorization", token)
        .json(&replay_req)
        .send()?
        .json::<Value>()?;
    //println!("{:?}", resp);

    let exit_code = resp["result"]["Receipt"]["ExitCode"].as_u64().unwrap_or(1);
    Ok(exit_code == 0)
}

/// Generic helper to call Lotus RPC methods and parse typed results
pub fn call_rpc<T: DeserializeOwned>(
    connection: &Connection,
    method: &str,
    params: &Value,
) -> Result<T> {
    let payload = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });

    let token = load_token_from_env()?; // same as used in `push_msg_to_mempool`

    let resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .header("Authorization", token)
        .json(&payload)
        .send()?
        .json::<Value>()?;

    if let Some(err) = resp.get("error") {
        Err(anyhow::anyhow!("RPC error in `{}`: {:?}", method, err))
    } else {
        let result = resp.get("result").ok_or_else(|| anyhow::anyhow!("Missing result field"))?;
        let typed = serde_json::from_value(result.clone())?;
        Ok(typed)
    }
}


fn encode_add_verified_client(client_bytes: &[u8], datacap: u64) -> Vec<u8> {
    let func = Function {
        name: "addVerifiedClient".to_string(),
        inputs: vec![
            Param { name: "client".to_string(), kind: ParamType::Bytes, internal_type: None },
            Param { name: "datacap".to_string(), kind: ParamType::Uint(256), internal_type: None },
        ],
        outputs: vec![],
        constant: None,
        state_mutability: ethers_core::abi::StateMutability::NonPayable,
    };

    let tokens = vec![
        Token::Bytes(client_bytes.to_vec()),
        Token::Uint(datacap.into()),
    ];

    func.encode_input(&tokens).expect("abi encode failed")
}