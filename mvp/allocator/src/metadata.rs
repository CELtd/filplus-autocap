use cid::Cid;
use fvm_shared::{ActorID, clock::ChainEpoch, piece::PaddedPieceSize};
use serde::{Serialize, Deserialize};
use serde_cbor::to_vec;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub provider: u64,
    pub data: Cid,
    pub size: PaddedPieceSize,
    pub term_min: ChainEpoch,
    pub term_max: ChainEpoch,
    pub expiration: ChainEpoch,
}

#[derive(Debug, Serialize)]
pub struct MetadataDisplay {
    pub provider: u64,
    pub data: String,
    pub size: u64,
    pub term_min: i64,
    pub term_max: i64,
    pub expiration: i64,
}

impl From<&Metadata> for MetadataDisplay {
    fn from(meta: &Metadata) -> Self {
        MetadataDisplay {
            provider: meta.provider,
            data: meta.data.to_string(),
            size: meta.size.0,
            term_min: meta.term_min,
            term_max: meta.term_max,
            expiration: meta.expiration,
        }
    }
}
