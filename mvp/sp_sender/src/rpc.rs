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

use crate::wallet::{self, Wallet};
use crate::metadata::{Metadata, serialize_metadata};

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


pub fn send_message_to(connection: &Connection, from: &Wallet, to: &str, amount_atto: &str, metadata: &Metadata) -> Result<String> {

    // Step 1: Fetch nonce
    let nonce = fetch_nonce(&connection, &from.address)?;

    // Step 2: Derive key
    let key = key_derive(&from.mnemonic, &from.derivation_path, "", &from.language)?;

    // Setp 3: Serialize the metadata
    let serialized = serialize_metadata(metadata); // CBOR-encoded bytes
    let params = RawBytes::new(serialized); // for the Message.params field 

    // Step 4: Build message
    let message = Message {
        version: 0,
        from: Address::from_str(&from.address.clone())?,
        to: Address::from_str(&to)?,
        sequence: nonce,
        value: TokenAmount::from_atto(amount_atto.parse::<u128>()?),
        method_num: 0,
        params: params,
        gas_limit: 2_000_0000,
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
/// Push a signed message to the Filecoin Mempool and return the CID string.
pub fn push_msg_to_mempool(connection: &Connection, push_msg: &Value) -> Result<String> {
    // Build the RPC request
    let push_req = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.MpoolPush",
        "params": [push_msg],
        "id": 1
    });

    // Send the request and parse response
    let push_resp = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
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
