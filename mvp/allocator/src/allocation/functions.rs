use anyhow::Result;

use fvm_ipld_encoding::{to_vec, RawBytes};
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::piece::PaddedPieceSize;
use cid::Cid;

use crate::constants::verifreg_actor::VERIFREG_ACTOR_ID;
use crate::allocation::types::{AllocationRequest, AllocationRequests, TransferParams};

/// Constructs the operator data payload (CBOR-encoded) needed for the `transfer_from`
/// call to the Verified Registry. This includes one allocation request with
/// metadata about the deal.
///
/// # Arguments
/// - `provider`: Actor ID of the storage provider as string (e.g., "1234").
/// - `piece_cid_str`: CID string of the deal (e.g., the Piece CID).
/// - `padded_size`: Size of the deal in padded bytes.
/// - `term_min`: Minimum term duration in epochs (currently unused—hardcoded).
/// - `term_max`: Maximum term duration in epochs (currently unused—hardcoded).
/// - `current_epoch`: Current chain epoch, used to calculate expiration.
pub fn craft_operator_data(
    provider: &str,
    piece_cid_str: &str,
    padded_size: &u64,
    term_min: &i64,
    term_max: &i64,
    expiraton: &i64,
    current_epoch: &u64,
) -> Result<RawBytes> {
    let piece_cid = Cid::try_from(piece_cid_str)?;

    // Construct the allocation request
    let alloc = AllocationRequest {
        provider: provider.parse()?,
        data: piece_cid,
        size: PaddedPieceSize(*padded_size),
        term_min: *term_min,
        term_max: *term_max,
        expiration: *current_epoch as i64 + *expiraton, // TODO: is it correct?
    };

    // Wrap in AllocationRequests
    let payload = AllocationRequests {
        allocations: vec![alloc],
        extensions: vec![], // Currently unused
    };

    Ok(RawBytes::new(to_vec(&payload)?))
}

/// Constructs the `TransferParams` needed to call `transfer_from`, using the
/// encoded operator data and the amount of datacap to send.
///
/// # Arguments
/// - `datacap_amount`: The amount to transfer (in unscaled FIL units, e.g., "32").
/// - `allocation_data`: CBOR-encoded `AllocationRequests` wrapped in RawBytes.
pub fn craft_transfer_params(
    datacap_amount: &str,
    allocation_data: RawBytes,
) -> Result<TransferParams> {
    // Scale from FIL to attoFIL (1e18)
    let bytes = datacap_amount.parse::<u128>()?;
    let scaled = bytes * 1_000_000_000_000_000_000u128;

    Ok(TransferParams {
        to: Address::new_id(VERIFREG_ACTOR_ID), // Verified Registry actor ID (f06)
        amount: TokenAmount::from_atto(scaled),
        operator_data: allocation_data,
    })
}

/// End-to-end utility to construct a `TransferParams` object from high-level
/// metadata fields. This wraps together operator data crafting and transfer param generation.
///
/// # Arguments
/// Same as `craft_operator_data` plus `datacap_amount` for the transfer.
pub fn craft_transfer_from_payload(
    provider_addr: &str,
    piece_cid_str: &str,
    padded_size: &u64,
    term_min: &i64,
    term_max: &i64,
    expiration: &i64,
    current_epoch: &u64,
    datacap_amount: &str,
) -> Result<TransferParams> {
    let operator_data = craft_operator_data(provider_addr, piece_cid_str, padded_size, term_min, term_max, expiration, current_epoch)?;
    craft_transfer_params(datacap_amount, operator_data)
}
