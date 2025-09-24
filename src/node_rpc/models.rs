use derive_more::Display;
use hex::FromHex;
use jsonrpsee::core::JsonValue;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

/// Block hash in nexadecimal format
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub struct BlockHash(pub String);

/// Block number in hexadecimal format
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub struct BlockNumber(pub String);

/// Block number in hexadecimal format
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub struct StorageKey(pub String);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub parent_hash: BlockHash,
    pub number: BlockNumber,
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
