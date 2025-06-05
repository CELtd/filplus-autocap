use anyhow::{Result, anyhow};
use serde_json::{json, Value};
use fvm_shared::address::Address;
use filecoin_signer::{key_derive, transaction_sign};
use fvm_shared::message::Message;
use std::str::FromStr;
use fvm_shared::econ::TokenAmount;
use fvm_ipld_encoding::{RawBytes, to_vec};
use base64::engine::general_purpose;
use base64::Engine as _;
use fvm_shared::ActorID;
use dotenv::dotenv;
use std::env;


use crate::wallet::{Wallet};
use crate::allocation::TransferParams;
use crate::rpc::Connection;
use crate::constants::datacap_actor::{DATACAP_ACTOR_ID, DATACAP_TRANSFER_FUNCTION_ID};

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
pub fn fetch_datacap_balance(connection: &Connection, address: &str) -> Result<String> {
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

    Ok(resp["result"].as_str().unwrap_or("0").to_string())
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

    // Step 3: Build message
    let message = Message {
        version: 0,
        from: Address::from_str(&from.address.clone())?,
        to: Address::from_str(&to)?,
        sequence: nonce,
        value: TokenAmount::from_atto(amount_atto.parse::<u128>()?),
        method_num: 0,
        params: RawBytes::new(vec![]),
        gas_limit: 1000000,
        gas_fee_cap: TokenAmount::from_atto("1000000000".parse::<u128>()?),
        gas_premium: TokenAmount::from_atto("1000000000".parse::<u128>()?),
    };

    // Step 4: Sign it
    let signed = transaction_sign(&message, &key.private_key)?;

    // Step 5: Build correct JSON structure manually
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

    // Step 6: Push the signed message
    let cid_str = push_msg_to_mempool(&connection, &push_msg)?;
    Ok(cid_str)

}

/// Create a datacap allocation transaction from a TransferParams object.
pub fn create_datacap_allocation(transfer_params:TransferParams, connection: &Connection, wallet: &Wallet) -> Result<String> {

    // Fetch nonce
    let nonce = fetch_nonce(&connection, &wallet.address)?;
    
    // Encode the parameters for the allocation
    let params_vec = to_vec(&transfer_params)?;
    let raw_params = RawBytes::new(params_vec.clone());

    // Create the message properly encoding the different fields
    let message = Message {
        version: 0,
        from: Address::from_str(&wallet.address)?,
        to: Address::new_id(DATACAP_ACTOR_ID),
        sequence: nonce,
        value: TokenAmount::from_atto(0u8),
        method_num: DATACAP_TRANSFER_FUNCTION_ID,
        params: raw_params,
        gas_limit: 20_000_0000,
        gas_fee_cap: TokenAmount::from_atto("2000000000".parse::<u128>()?),
        gas_premium: TokenAmount::from_atto("2000000000".parse::<u128>()?),
    };

    // Derive key and sign it
    let key = key_derive(&wallet.mnemonic, &wallet.derivation_path, "", &wallet.language)?;
    let signed = transaction_sign(&message, &key.private_key)?;

    // Build proper JSON
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

    // Push message
    let cid = push_msg_to_mempool(&connection, &push_msg)?;
    Ok(cid)

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
    println!("{:?}", resp);

    let exit_code = resp["result"]["Receipt"]["ExitCode"].as_u64().unwrap_or(1);
    Ok(exit_code == 0)
}

