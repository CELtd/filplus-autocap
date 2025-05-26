use std::str::FromStr;
use anyhow::Result;

use fvm_ipld_encoding::{to_vec, RawBytes, Cbor};
use fvm_ipld_encoding::tuple::*;
use fvm_ipld_encoding::serde_bytes;
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::{ActorID, clock::ChainEpoch, piece::PaddedPieceSize};
use cid::Cid;
use multibase::decode;

use crate::constants::EPOCHS_PER_DAY;

pub type ClaimExtensionRequest = ();
//I am working on Filecoin tooling using fvm_ipld_encoding (0.3.3), which expects Cid from the cid 0.10 crate. 
//However, my dependency tree includes filecoin-signer and various fil_actor_* crates, which pull in cid 0.8.6. 
//This causes a type mismatch: the Cid used in my code is not the same as the one expected by fvm_ipld_encoding, so Serialize_tuple fails. 
//I need a way to ensure that only cid 0.10 is used throughout the dependency graph, or to patch the older dependencies to unify on the correct version.
#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct AllocationRequest {
    pub provider: ActorID,
    pub data: Cid, 
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


pub fn craft_operator_data(
    provider: &str,
    piece_cid_str: &str,
    padded_size: u64,
    current_epoch: u64,
) -> Result<RawBytes> {

    let piece_cid = Cid::try_from(piece_cid_str)?;
    //let multihash_bytes = piece_cid.hash().to_bytes();
    //println!("Multihash bytes: {:02x?}", multihash_bytes);

    let alloc = AllocationRequest {
        provider: provider.parse()?,
        data: piece_cid,
        size: PaddedPieceSize(padded_size),
        term_min: 180 * EPOCHS_PER_DAY,
        term_max: 540 * EPOCHS_PER_DAY,
        expiration: current_epoch as i64 + 60 * EPOCHS_PER_DAY,
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
        to: Address::new_id(6),
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
