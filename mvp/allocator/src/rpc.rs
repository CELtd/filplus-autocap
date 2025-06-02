use anyhow::{Result, anyhow};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use fvm_shared::address::Address;
use filecoin_signer::{key_derive, transaction_sign};
use fvm_shared::message::Message;
use std::str::FromStr;
use fvm_shared::econ::TokenAmount;
use fvm_ipld_encoding::{RawBytes, to_vec};
use base64::engine::general_purpose;
use base64::Engine as _;
use serde::Serialize;
use fvm_shared::ActorID;
use dotenv::dotenv;
use std::env;


use crate::wallet::{self, Wallet};
use crate::allocation::TransferParams;

//Connection struct to perform JSON-RPC requests
pub struct Connection {
    pub client: Client,
    pub rpc_url: String,
}
impl Connection {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: Client::new(),
            rpc_url: rpc_url.to_string(),
        }
    }
}

// Load lotus jwt token for local devnet
fn load_token_from_env() -> Result<String, anyhow::Error> {
    dotenv().ok(); // load from .env
    let token = env::var("LOTUS_JWT")?;
    Ok(format!("Bearer {}", token))
}

/// Get the current head block number from the Filecoin JSON-RPC
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

/// Fetch full block with all transactions
pub fn get_block_info(connection: &Connection, block_number: &u64) -> Result<Value> {
    // 1. Get TipSetKey at height
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

    // 2. Request all messages in the tipset
    let messages_req = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.ChainGetMessagesInTipset",
        "params": [cid_array],
        "id": 1
    });

    let messages_resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .json(&messages_req)
        .send()?
        .json::<Value>()?;

    Ok(messages_resp["result"].clone())
}


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

/// Resolves a Filecoin address (like `f1...`, `t1...`, etc.) to its ID address (like `f0...`)
/// and returns the numeric ActorID.
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

// To check
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

pub fn fetch_datacap_allowance(client: &Client, address: &str, rpc_url: &str) -> Result<String> {

    #[derive(Serialize)]
    struct GetAllowanceParams(Address, Address);
    
    // Step 1: Set up the parameters
    let params = GetAllowanceParams (
        Address::from_str("f410fo4l64ogb7kay3bbqmbbj6tpsxlz3eanoqs6nj6q")?, 
        Address::from_str("f410fo4l64ogb7kay3bbqmbbj6tpsxlz3eanoqs6nj6q")?
    );
    
    let cbor_params = serde_cbor::to_vec(&params)?;

    // Step 2 Build message
    let message = json!({
        "To": Address::new_id(7).to_string(),  // DataCap actor
        "From": Address::from_str("f410furmiftbvstlz3xvjccqdo3376lssdu752ietvpa")?.to_string(), // could be any valid address
        "Method": serde_json::Value::from(4205072950u64), // allowance()
        "Params":  general_purpose::STANDARD.encode(cbor_params.to_vec()),
        "Value": "0"
    });


    let payload = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.StateCall",
        "params": [message, null],
        "id": 1
    });
    println!("\n{:?}\n", payload);


    let resp = client.post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?
        .json::<serde_json::Value>()?;
    println!("{:?}", resp);

    Ok(resp["result"].as_str().unwrap_or("0").to_string())
}



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

//pub fn query_filecoin_actor(client: &Client, from: &Wallet, to_actor_id: &Address, amount_atto: &str, method_num : &u128, rpc_url: &str) -> Result<String> {
//    
//    // Step 1: Fetch nonce
//    let nonce = fetch_nonce(&client, &of.address, &rpc_url)?;
//    
//
//    // Step 2: Derive key and encode your address
//    let key = key_derive(&of.mnemonic, &of.derivation_path, "", &of.language)?;
//    let addr = Address::from_str(&of.address.clone())?;
//    let params = RawBytes::new(to_vec(&addr)?);
//    //println!("push_msg: {:#?}", Address::new_id(7));
//
//    // Step 3: Build the message
//    let message = Message {
//        version: 0,
//        to: Address::new_id(7), //Datacap Actor id
//        from: addr.clone(),
//        sequence: nonce,
//        value: TokenAmount::from_atto(0u8),
//        method_num: 3261979605, // Balance
//        params,
//        gas_limit: 3000000,
//        gas_fee_cap: TokenAmount::from_atto("5000000000".parse::<u128>()?),
//        gas_premium: TokenAmount::from_atto("5000000000".parse::<u128>()?),
//    };
//
//    // Step 4: Sign it
//    let signed = transaction_sign(&message, &key.private_key)?;
//
//    // Step 5: Build proper JSON
//    let push_msg = json!({
//        "Message": {
//            "Version": message.version,
//            "To": message.to.to_string(),
//            "From": message.from.to_string(),
//            "Nonce": message.sequence,
//            "Value": message.value.atto().to_string(),
//            "GasLimit": message.gas_limit,
//            "GasFeeCap": message.gas_fee_cap.atto().to_string(),
//            "GasPremium": message.gas_premium.atto().to_string(),
//            "Method": message.method_num,
//            "Params": general_purpose::STANDARD.encode(message.params.to_vec()),
//        },
//        "Signature": {
//            "Type": signed.signature.signature_type() as u8,
//            "Data": general_purpose::STANDARD.encode(signed.signature.bytes()),
//        }
//    });
//
//    // Push message
//    let cid = push_msg_to_mempool(&client, &rpc_url, &push_msg)?;
//    Ok(cid)
//}

pub fn create_datacap_allocation(transfer_params:TransferParams, connection: &Connection, wallet: &Wallet) -> Result<String> {

    
    const DATACAP_ACTOR_ID: u64 = 7;

    // Step 1: Fetch nonce
    let nonce = fetch_nonce(&connection, &wallet.address)?;
    

    // Step 2: Derive key and encode your address
    let key = key_derive(&wallet.mnemonic, &wallet.derivation_path, "", &wallet.language)?;
    let addr = Address::from_str(&wallet.address.clone())?;
    //let params_bytes = RawBytes::new(to_vec(&transfer_params)?);
    let params_vec = to_vec(&transfer_params)?;
    let raw_params = RawBytes::new(params_vec.clone()); // ✅ only wrap here
    let message = Message {
        version: 0,
        from: Address::from_str(&wallet.address)?,
        to: Address::new_id(7), // f07 = Datacap Actor
        sequence: nonce,
        value: TokenAmount::from_atto(0u8),
        method_num: 80475954, // transfer
        params: raw_params,
        gas_limit: 20_000_0000,
        gas_fee_cap: TokenAmount::from_atto("2000000000".parse::<u128>()?),
        gas_premium: TokenAmount::from_atto("2000000000".parse::<u128>()?),
    };

    // Step 4: Sign it
    let signed = transaction_sign(&message, &key.private_key)?;

    // Step 5: Build proper JSON
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


pub fn get_datacap_balance_as_tx(connection: &Connection, of: &Wallet) -> Result<String> {
    
    const DATACAP_ACTOR_ID: u64 = 7;

    // Step 1: Fetch nonce
    let nonce = fetch_nonce(&connection, &of.address)?;
    

    // Step 2: Derive key and encode your address
    let key = key_derive(&of.mnemonic, &of.derivation_path, "", &of.language)?;
    let addr = Address::from_str(&of.address.clone())?;
    let params = RawBytes::new(to_vec(&addr)?);
    //println!("push_msg: {:#?}", Address::new_id(7));

    // Step 3: Build the message
    let message = Message {
        version: 0,
        to: Address::new_id(DATACAP_ACTOR_ID), //Datacap Actor id
        from: addr.clone(),
        sequence: nonce,
        value: TokenAmount::from_atto(0u8),
        method_num: 3261979605, // Balance
        params,
        gas_limit: 3000000,
        gas_fee_cap: TokenAmount::from_atto("5000000000".parse::<u128>()?),
        gas_premium: TokenAmount::from_atto("5000000000".parse::<u128>()?),
    };

    // Step 4: Sign it
    let signed = transaction_sign(&message, &key.private_key)?;

    // Step 5: Build proper JSON
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

    // Push message
    let cid = push_msg_to_mempool(&connection, &push_msg)?;
    Ok(cid)
}

//pub fn send_datacap_transfer(
//    connection: &Connection,
//    from: &Wallet,
//    amount: &str, // in atto
//    operator_data: Vec<u8>, // CBOR-encoded AllocationRequests
//) -> Result<String> {
//    const VERIFIED_REGISTRY_ACTOR_ID: u64 = 6;
//    const TRANSFER_FROM_METHOD: u64 = 3621052141;
//
//    let nonce = fetch_nonce(&connection, &from.address)?;
//    let key = key_derive(&from.mnemonic, &from.derivation_path, "", &from.language)?;
//
//    let params = RawBytes::new(operator_data);
//
//    let message = Message {
//        version: 0,
//        to: Address::new_id(VERIFIED_REGISTRY_ACTOR_ID),
//        from: Address::from_str(&from.address)?,
//        sequence: nonce,
//        value: TokenAmount::from_atto(amount.parse::<u128>()?),
//        method_num: TRANSFER_FROM_METHOD,
//        params,
//        gas_limit: 20_000_000,
//        gas_fee_cap: TokenAmount::from_atto("5000000000".parse::<u128>()?),
//        gas_premium: TokenAmount::from_atto("5000000000".parse::<u128>()?),
//    };
//
//    let signed = transaction_sign(&message, &key.private_key)?;
//
//    let push_msg = json!({
//        "Message": {
//            "Version": message.version,
//            "To": message.to.to_string(),
//            "From": message.from.to_string(),
//            "Nonce": message.sequence,
//            "Value": message.value.atto().to_string(),
//            "GasLimit": message.gas_limit,
//            "GasFeeCap": message.gas_fee_cap.atto().to_string(),
//            "GasPremium": message.gas_premium.atto().to_string(),
//            "Method": message.method_num,
//            "Params": general_purpose::STANDARD.encode(message.params.to_vec()),
//        },
//        "Signature": {
//            "Type": signed.signature.signature_type() as u8,
//            "Data": general_purpose::STANDARD.encode(signed.signature.bytes()),
//        }
//    });
//
//    push_msg_to_mempool(&connection, &push_msg)
//}



/// Push a signed message to the Filecoin Mempool and return the CID string.
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