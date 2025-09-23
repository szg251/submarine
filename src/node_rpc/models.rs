use derive_more::Display;
use jsonrpsee::core::JsonValue;
use serde::Deserialize;

use crate::node_rpc::error::ChainMetadataDeserializationError;

/// Block hash is hexadecimal format
#[derive(Debug, Deserialize, Display)]
pub struct BlockHash(pub String);

/// Block number in hexadecimal format
#[derive(Debug, Deserialize, Display)]
pub struct BlockNumber(pub String);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub parent_hash: BlockHash,
    pub number: BlockNumber,
    pub state_root: String,
    pub extrinsics_root: String,
    pub digest: Digest,
}

#[derive(Debug, Deserialize)]
pub struct Digest {
    pub logs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SignedBlock {
    pub block: Block,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub extrinsics: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
    pub spec_name: String,
    pub impl_name: String,
    pub apis: Vec<JsonValue>,
    pub spec_version: u64,
}

#[derive(Debug)]
pub struct ChainMetadata {
    pub version: u8,
    // pub magic: u32,
    pub data: Vec<u8>,
}

impl ChainMetadata {
    /// Decodes a chain metadata object from raw bytes
    /// For performance optimization, this will consume the original vector
    pub fn decode(mut bytes: Vec<u8>) -> Result<Self, ChainMetadataDeserializationError> {
        if bytes.len() < 5 {
            return Err(ChainMetadataDeserializationError::BytestringTooShort {
                length: bytes.len(),
            });
        }

        // let magic_bytes: [u8; 4] = (&bytes[0..4]).try_into().unwrap();
        // let magic = u32::from_le_bytes(magic_bytes);

        // The version is the 5th byte (index 4) after the 4-byte magic number ('meta').
        let version = bytes[4];

        bytes.drain(..5);

        Ok(Self {
            version,
            // magic,
            data: bytes,
        })
    }
}
