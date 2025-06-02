use fvm_ipld_encoding::{RawBytes};
use fvm_ipld_encoding::tuple::*;
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::{ActorID, clock::ChainEpoch, piece::PaddedPieceSize};
use cid::Cid;

pub type ClaimExtensionRequest = ();

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone)]
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
