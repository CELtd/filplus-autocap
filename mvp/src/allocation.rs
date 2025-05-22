use fvm_shared::address::Address;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use anyhow::Result;
use cid::Cid;
use fvm_ipld_encoding::to_vec;

#[derive(Serialize, Deserialize)]
pub struct Size(pub u64);

#[derive(Serialize, Deserialize)]
pub struct AllocationRequest {
    pub provider: Address,
    pub data: Vec<u8>, // piece CID (commP)
    pub size: Size,
    pub term_min: i64,
    pub term_max: i64,
    pub expiration: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AllocationRequests {
    pub allocations: Vec<AllocationRequest>,
    pub extensions: Vec<()>, // unused
}

pub fn craft_allocation_request(
    provider: &str,
    piece_cid_str: &str,
    padded_size: u64,
    current_epoch: i64,
) -> Result<Vec<u8>> {
    let provider_addr = Address::from_str(provider)?;
    let piece_cid = Cid::from_str(piece_cid_str)?;
    let piece_bytes = piece_cid.to_bytes();

    let alloc = AllocationRequest {
        provider: provider_addr,
        data: piece_bytes,
        size: Size(padded_size),
        term_min: 180 * 2880, // 180 days
        term_max: 540 * 2880, // 540 days
        expiration: current_epoch + 600 * 2880,
    };

    let payload = AllocationRequests {
        allocations: vec![alloc],
        extensions: vec![],
    };

    Ok(to_vec(&payload)?)
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromParams {
    pub from: Address,         // Your dummy client (f4...) 
    pub to: Address,           // The Verified Registry actor (f06)
    pub amount: String,        // Datacap amount as a string (attoFIL style)
    pub operator_data: Vec<u8> // CBOR-encoded AllocationRequests
}


pub fn build_transfer_from_payload(
    from_addr: &str,              // dummy verified client (f4...)
    datacap_amount: &str,         // in atto style string, e.g. "1024"
    allocation_cbor: Vec<u8>      // result from `craft_allocation_request_from_dummy`
) -> Result<TransferFromParams> {
    Ok(TransferFromParams {
        from: Address::from_str(from_addr)?,
        to: Address::new_id(6), // Verified registry
        amount: datacap_amount.to_string(),
        operator_data: allocation_cbor,
    })
}
