use fvm_shared::address::Address;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use anyhow::Result;
use cid::Cid;
use fvm_ipld_encoding::to_vec;
use serde_tuple::{Serialize_tuple, Deserialize_tuple};
use fvm_shared::econ::TokenAmount;
use fvm_ipld_encoding::RawBytes;
use fvm_ipld_encoding::from_slice;

#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct Size {
    pub size: u64,
}


#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct AllocationRequest {
    pub provider: Address,
    pub data: Vec<u8>, // piece CID (commP)
    pub size: Size,
    pub term_min: i64,
    pub term_max: i64,
    pub expiration: u64,
}


#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct AllocationRequests {
    pub allocations: Vec<AllocationRequest>,
    pub extensions: Vec<()>, // unused
}

#[derive(Debug)]
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct TransferParams {
    pub to: Address,           // The Verified Registry actor (f06)
    pub amount: TokenAmount,        // Datacap amount as a string (attoFIL style)
    pub operator_data: RawBytes // CBOR-encoded AllocationRequests
}

pub fn craft_operator_data(
    provider: &str,
    piece_cid_str: &str,
    padded_size: u64,
    current_epoch: u64,
) -> Result<RawBytes> {
    let provider_addr = Address::from_str(provider)?;
    let piece_cid = Cid::from_str(piece_cid_str)?;
    let piece_bytes = piece_cid.to_bytes();

    let alloc = AllocationRequest {
        provider: provider_addr,
        data: piece_bytes,
        size: Size{size: padded_size},
        term_min: 180 * 2880, // 180 days
        term_max: 540 * 2880, // 540 days
        expiration: current_epoch + 600 * 2880,
    };

    let payload = AllocationRequests {
        allocations: vec![alloc],
        extensions: vec![],
    };

    let cbor = to_vec(&payload)?;               // ← this must start with 0x82
    let raw = RawBytes::new(cbor);
    Ok(raw)
}



pub fn craft_transfer_params(
    datacap_amount: &str,         // in atto style string, e.g. "1024"
    allocation_data: RawBytes      // result from `craft_allocation_request_from_dummy`
) -> Result<TransferParams> {
    Ok(TransferParams {
        to: Address::new_id(6), // Verified registry address
        amount: TokenAmount::from_atto(datacap_amount.parse::<u128>()?),
        operator_data: allocation_data,
    })
}

pub fn craft_transfer_from_payload(
    provider_addr: &str,      // Provider (f0...)
    piece_cid_str: &str,      // CID string (commP)
    padded_size: u64,         // Piece size in bytes
    current_epoch: u64,       // Current chain epoch
    datacap_amount: &str      // Amount of datacap to transfer (in attoFIL string)
//) -> Result<Vec<u8>> {
) -> Result<TransferParams>{
    // 1. Build operator_data from allocation request
    let operator_data = craft_operator_data(provider_addr, piece_cid_str, padded_size, current_epoch)?;

    // 2. Wrap it in transfer_from params
    let transfer_params = craft_transfer_params(datacap_amount, operator_data)?;
    Ok(transfer_params)
}
