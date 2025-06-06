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
use serde::de::DeserializeOwned;

use crate::wallet::{self, Wallet};
use crate::metadata::{Metadata, serialize_metadata, CorruptedMetadata, serialize_corrupted_metadata};

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

/// Load Lotus devnet JWT token from environment.
fn load_token_from_env() -> Result<String, anyhow::Error> {
    dotenv().ok(); // load from .env
    let token = env::var("LOTUS_JWT")?;
    Ok(format!("Bearer {}", token))
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


/// Import a secp256k1 private key into a local Lotus node
pub fn import_wallet_key(connection: &Connection, private_key_base64: &str) -> Result<String> {

    // Load JWT token (assumes local devnet)
    let token = load_token_from_env()?;

    // Build request payload
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "Filecoin.WalletImport",
        "params": [
            {
                "Type": "secp256k1",
                "PrivateKey": private_key_base64
            }
        ],
        "id": 1
    });

    let response = connection.client.post(&connection.rpc_url)
        .header("Content-Type", "application/json")
        .header("Authorization", token) // Only for devnet
        .json(&payload)
        .send()?
        .json::<Value>()?;
    
    // Handle response
    if let Some(error) = response.get("error") {
        let message = error.get("message").and_then(|m| m.as_str()).unwrap_or("");
        if message.contains("key already exists") {
            println!("✅ Key already exists in Lotus wallet. Proceeding.");
            return Ok("already imported".to_string());
        } else {
            return Err(anyhow!("❌ Wallet import failed: {}", message));
        }
    }

    // If successful, return imported address or raw result
    Ok(response.to_string())
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

pub fn send_metadata_tx(
    connection: &Connection,
    wallet: &Wallet,
    to_address: &str,
    amount_fil: f64,
    metadata: &Metadata,
) -> Result<String> {
    let amount_atto = (amount_fil * 1e18).round() as u128;

    let cbor = serde_cbor::to_vec(metadata)?;
    let params = RawBytes::new(cbor.clone());

    let key = key_derive(&wallet.mnemonic, &wallet.derivation_path, "", &wallet.language)?;
    let nonce = fetch_nonce(connection, &wallet.address)?;

    // === Estimate gas_premium ===
    let gas_limit = 1_362_763 ;

    let gas_premium: String = call_rpc(
        connection,
        "Filecoin.GasEstimateGasPremium",
        &serde_json::json!([
            0,
            wallet.address,
            gas_limit,
            null
        ]),
    )?;
    let gas_premium_atto = gas_premium.parse::<u128>()?;

    // === Build temp message to estimate fee cap ===
    let fee_cap: String = call_rpc(
        connection,
        "Filecoin.GasEstimateFeeCap",
        &serde_json::json!([
            {
                "Version": 0,
                "To": to_address,
                "From": wallet.address,
                "Nonce": nonce,
                "Value": amount_atto.to_string(),
                "GasLimit": gas_limit,
                "GasFeeCap": "0",
                "GasPremium": gas_premium_atto.to_string(),
                "Method": 0,
                "Params": params
            },
            0,
            null
        ]),
    )?;
    let gas_fee_cap_atto = fee_cap.parse::<u128>()?;

    // === Final message ===
    let message = Message {
        version: 0,
        from: Address::from_str(&wallet.address)?,
        to: Address::from_str(to_address)?,
        sequence: nonce,
        value: TokenAmount::from_atto(amount_atto),
        method_num: 0,
        params,
        gas_limit,
        gas_fee_cap: TokenAmount::from_atto(gas_fee_cap_atto),
        gas_premium: TokenAmount::from_atto(gas_premium_atto),
    };

    let signed = transaction_sign(&message, &key.private_key)?;

    let push_msg = serde_json::json!({
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
            "Params": general_purpose::STANDARD.encode(cbor),
        },
        "Signature": {
            "Type": signed.signature.signature_type() as u8,
            "Data": general_purpose::STANDARD.encode(signed.signature.bytes()),
        }
    });

    let cid = crate::rpc::push_msg_to_mempool(connection, &push_msg)?;
    Ok(cid)
}


pub fn send_corrupted_metadata_tx(
    connection: &Connection,
    wallet: &Wallet,
    to_address: &str,
    amount_fil: f64, // Float FIL value
    metadata: &CorruptedMetadata,
) -> Result<String> {
    // Convert FIL to attoFIL (1 FIL = 10^18 attoFIL)
    let amount_atto = (amount_fil * 1e18).round() as u128;

    // Serialize metadata
    let cbor = serde_cbor::to_vec(metadata)?;
    let params = RawBytes::new(cbor.clone());

    // Derive key and fetch nonce
    let key = key_derive(&wallet.mnemonic, &wallet.derivation_path, "", &wallet.language)?;
    let nonce = fetch_nonce(connection, &wallet.address)?;

    let message = Message {
        version: 0,
        from: Address::from_str(&wallet.address)?,
        to: Address::from_str(to_address)?,
        sequence: nonce,
        value: TokenAmount::from_atto(amount_atto),
        method_num: 0,
        params,
        gas_limit: 20_000_000,
        gas_fee_cap: TokenAmount::from_atto("1000000000".parse::<u128>()?),
        gas_premium: TokenAmount::from_atto("1000000000".parse::<u128>()?),
    };

    let signed = transaction_sign(&message, &key.private_key)?;

    let push_msg = serde_json::json!({
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
            "Params": general_purpose::STANDARD.encode(cbor),
        },
        "Signature": {
            "Type": signed.signature.signature_type() as u8,
            "Data": general_purpose::STANDARD.encode(signed.signature.bytes()),
        }
    });

    let cid = crate::rpc::push_msg_to_mempool(connection, &push_msg)?;
    Ok(cid)
}
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