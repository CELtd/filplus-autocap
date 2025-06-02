use anyhow::Result;

use fvm_ipld_encoding::{to_vec, RawBytes};
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::{piece::PaddedPieceSize};
use cid::Cid;

use crate::constants::filecoin::EPOCHS_PER_DAY;
use crate::allocation::types::{AllocationRequest, AllocationRequests, TransferParams};

pub fn craft_operator_data(
    provider: &str,
    piece_cid_str: &str,
    padded_size: &u64,
    term_min: &i64,
    term_max: &i64,
    current_epoch: &u64,
) -> Result<RawBytes> {

    let piece_cid = Cid::try_from(piece_cid_str)?;
    //let multihash_bytes = piece_cid.hash().to_bytes();
    //println!("Multihash bytes: {:02x?}", multihash_bytes);

    let alloc = AllocationRequest {
        provider: provider.parse()?,
        data: piece_cid,
        size: PaddedPieceSize(*padded_size),
        //term_min: *term_min,
        //term_max: *term_max,
        term_min : 518400, //TODO
        term_max : 2 * 518400, //TODO
        expiration: *current_epoch as i64 + 1 * EPOCHS_PER_DAY, //TODO fix this dummy 1
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
    let bytes = datacap_amount.parse::<u128>()?;
    let scaled = bytes * 1_000_000_000_000_000_000u128;
    Ok(TransferParams {
        to: Address::new_id(6),
        amount: TokenAmount::from_atto(scaled),
        operator_data: allocation_data,
    })
}

pub fn craft_transfer_from_payload(
    provider_addr: &str,
    piece_cid_str: &str,
    padded_size: &u64,
    term_min: &i64,
    term_max: &i64,
    current_epoch: &u64,
    datacap_amount: &str,
) -> Result<TransferParams> {
    let operator_data = craft_operator_data(provider_addr, piece_cid_str, padded_size, term_min, term_max, current_epoch)?;
    //println!("Raw CBOR: {:02x?}", operator_data.to_vec());
    craft_transfer_params(datacap_amount, operator_data)
}

