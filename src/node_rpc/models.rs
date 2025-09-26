#![allow(dead_code)]
use derive_more::Display;
use hex::FromHex;
use jsonrpsee::core::JsonValue;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

/// Block hash as hexadecimal string
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
pub struct BlockHashHex(pub String);

/// Block number as hexadecimal string
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
pub struct BlockNumberHex(pub String);

impl From<u64> for BlockNumberHex {
    fn from(value: u64) -> Self {
        Self(format!("0x{value:x}"))
    }
}

/// Block number as hexadecimal string
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
pub struct StorageKeyHex(pub String);

impl From<Vec<u8>> for StorageKeyHex {
    fn from(value: Vec<u8>) -> Self {
        Self(format!("0x{}", hex::encode(value)))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub parent_hash: BlockHashHex,
    pub number: BlockNumberHex,
    pub state_root: String,
    pub extrinsics_root: String,
    pub digest: Digest,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Digest {
    pub logs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignedBlock {
    pub block: Block,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub extrinsics: Vec<ExtrinsicBytes>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct ExtrinsicBytes(#[serde(deserialize_with = "deserialize_hex")] pub Vec<u8>);

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct StorageValueBytes(#[serde(deserialize_with = "deserialize_hex")] pub Vec<u8>);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
    pub spec_name: String,
    pub impl_name: String,
    pub apis: Vec<JsonValue>,
    pub spec_version: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncState {
    pub starting_block: u64,
    pub current_block: u64,
    pub highest_block: u64,
}

/// Chain Metadata as a bytestring
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct ChainMetadataBytes(#[serde(deserialize_with = "deserialize_hex")] pub Vec<u8>);

/// Deserialize a hexadecimal string with an optional 0x prefix info a bytestring
pub fn deserialize_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    let s = s.strip_prefix("0x").unwrap_or(&s);

    Vec::from_hex(s).map_err(|e| D::Error::custom(format!("Hex decoding error: {e}")))
}
