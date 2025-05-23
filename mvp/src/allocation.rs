use std::str::FromStr;
use anyhow::Result;

use fvm_ipld_encoding::{to_vec, RawBytes};
use fvm_ipld_encoding::tuple::*;
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::{ActorID, clock::ChainEpoch, piece::PaddedPieceSize};
use multibase::Base;
use multihash::Multihash;

pub type ClaimExtensionRequest = ();

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct AllocationRequest {
    pub provider: ActorID,
    pub data: Vec<u8>, // manually serialized piece CID
    pub size: PaddedPieceSize,
    pub term_min: ChainEpoch,
    pub term_max: ChainEpoch,
    pub expiration: ChainEpoch,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct AllocationRequests {
    pub allocations: Vec<AllocationRequest>,
    pub extensions: Vec<ClaimExtensionRequest>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct TransferParams {
    pub to: Address,
    pub amount: TokenAmount,
    pub operator_data: RawBytes,
}

fn parse_base32_cid_to_bytes(cid_str: &str) -> Result<Vec<u8>> {
    let (_base, decoded) = multibase::decode(cid_str)?;
    Ok(decoded)
}

pub fn craft_operator_data(
    provider: &str,
    piece_cid_str: &str,
    padded_size: u64,
    current_epoch: u64,
) -> Result<RawBytes> {

    let provider_id: ActorID = provider[2..].parse()?; // provider like f01234
    //let piece_cid_bytes = parse_base32_cid_to_bytes(piece_cid_str)?;
    let piece_cid_bytes = multibase::decode(piece_cid_str)
    .map_err(|e| anyhow::anyhow!("Failed to decode multibase CID: {}", e))?
    .1;

    let alloc = AllocationRequest {
        provider: provider_id,
        data: piece_cid_bytes,
        size: PaddedPieceSize(padded_size),
        term_min: 180 * 2880,
        term_max: 540 * 2880,
        expiration: current_epoch as ChainEpoch + 600 * 2880,
    };

    let payload = AllocationRequests {
        allocations: vec![alloc],
        extensions: vec![],
    };

    Ok(RawBytes::new(to_vec(&payload)?))
}

pub fn craft_transfer_params(
    datacap_amount: &str,
    allocation_data: RawBytes,
) -> Result<TransferParams> {
    Ok(TransferParams {
        to: Address::new_id(6), // f06
        amount: TokenAmount::from_atto(datacap_amount.parse::<u128>()?),
        operator_data: allocation_data,
    })
}

pub fn craft_transfer_from_payload(
    provider_addr: &str,
    piece_cid_str: &str,
    padded_size: u64,
    current_epoch: u64,
    datacap_amount: &str,
) -> Result<TransferParams> {
    let operator_data = craft_operator_data(provider_addr, piece_cid_str, padded_size, current_epoch)?;
    println!("Raw CBOR: {:02x?}", operator_data.to_vec());
    craft_transfer_params(datacap_amount, operator_data)
}
