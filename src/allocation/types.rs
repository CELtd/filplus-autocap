use fvm_ipld_encoding::{RawBytes};
use fvm_ipld_encoding::tuple::*;
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::{ActorID, clock::ChainEpoch, piece::PaddedPieceSize};
use cid::Cid;

/// Placeholder type for future support of allocation extensions.
/// Currently unused.
pub type ClaimExtensionRequest = ();

/// Represents a single datacap allocation request.
///
/// This corresponds to a deal that a storage provider wants to verify.
/// It includes deal metadata such as the piece CID, size, duration, and expiration.
#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone)]
pub struct AllocationRequest {
    pub provider: ActorID,            // SP actor ID
    pub data: Cid,                    // Piece CID
    pub size: PaddedPieceSize,        // Size in padded bytes
    pub term_min: ChainEpoch,         // Minimum duration in epochs
    pub term_max: ChainEpoch,         // Maximum duration in epochs
    pub expiration: ChainEpoch,       // Expiry epoch for the allocation
}

/// Wrapper struct to send multiple allocation requests at once,
/// along with any extension requests (currently empty).
#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct AllocationRequests {
    pub allocations: Vec<AllocationRequest>,
    pub extensions: Vec<ClaimExtensionRequest>, // Unused for now
}

/// Parameters for calling `transfer` on the Verified Registry actor.
///
/// This struct wraps the target address (the Verified Registry),
/// the amount of datacap to transfer, and the operator data (which is a
/// serialized `AllocationRequests` payload).
#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct TransferParams {
    pub to: Address,             // Recipient (e.g., Verified Registry actor)
    pub amount: TokenAmount,    // Amount of datacap to send
    pub operator_data: RawBytes // Serialized allocation requests
}
