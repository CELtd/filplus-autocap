use cid::Cid;
use fvm_shared::{ActorID, clock::ChainEpoch, piece::PaddedPieceSize};
use serde::{Serialize, Deserialize};
use serde_cbor::to_vec;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub provider: u64,
    pub data: Cid,
    pub size: PaddedPieceSize,
    pub term_min: ChainEpoch,
    pub term_max: ChainEpoch,
    pub expiration: ChainEpoch,
}

pub fn serialize_metadata(meta: &Metadata) -> Vec<u8> {
    to_vec(meta).expect("failed to serialize metadata")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorruptedMetadata {
    pub provider: u64,
    pub data: Cid,
    pub size: PaddedPieceSize,
    pub term_min: ChainEpoch,
    pub term_max: ChainEpoch,
}

pub fn serialize_corrupted_metadata(meta: &CorruptedMetadata) -> Vec<u8> {
    to_vec(meta).expect("failed to serialize metadata")
}