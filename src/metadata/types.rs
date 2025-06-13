use cid::Cid;
use fvm_shared::{clock::ChainEpoch, piece::PaddedPieceSize};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub provider: u64,
    pub data: Cid,
    pub size: PaddedPieceSize,
    pub term_min: ChainEpoch,
    pub term_max: ChainEpoch,
    pub expiration: ChainEpoch,
}
impl From<MetadataDisplay> for Metadata {
    fn from(display: MetadataDisplay) -> Self {
        Metadata {
            provider: display.provider,
            data: Cid::try_from(display.data.as_str())
                .expect("Invalid CID in display metadata"),
            size: PaddedPieceSize(display.size),
            term_min: display.term_min,
            term_max: display.term_max,
            expiration: display.expiration,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
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


