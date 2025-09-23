use parity_scale_codec::Decode;

pub use super::v11::{
    ErrorMetadata, EventMetadata, ExtrinsicMetadata, FunctionArgumentMetadata, FunctionMetadata,
    Metadata, ModuleConstantMetadata, ModuleMetadata, StorageEntryModifier, StorageHasher, Type,
};

#[derive(Debug, Decode)]
pub struct StorageEntryMetadata {
    pub name: String,
    pub modifier: StorageEntryModifier,
    pub type_: StorageEntryType,
    pub fallback: Vec<u8>,
    pub docs: Vec<String>,
}

#[derive(Debug, Decode)]
pub enum StorageEntryType {
    Plain(Type),
    Map(StorageEntryMap),
    DoubleMap(StorageEntryDoubleMap),
    NMap(StorageEntryNMap),
}

#[derive(Debug, Decode)]
pub struct StorageEntryMap {
    pub hasher: StorageHasher,
    pub key: Type,
    pub value: Type,
    pub linked: bool,
}

#[derive(Debug, Decode)]
pub struct StorageEntryDoubleMap {
    pub hasher: StorageHasher,
    pub key1: Type,
    pub key2: Type,
    pub value: Type,
    pub key2hasher: StorageHasher,
}

#[derive(Debug, Decode)]
pub struct StorageEntryNMap {
    pub key_vec: Vec<Type>,
    pub hashers: StorageHasher,
    pub value: Type,
}

#[derive(Debug, Decode)]
pub struct StorageMetadata {
    pub prefix: String,
    pub items: Vec<StorageEntryMetadata>,
}
