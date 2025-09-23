use parity_scale_codec::Decode;

pub use super::v11::{
    ErrorMetadata, EventMetadata, ExtrinsicMetadata, FunctionArgumentMetadata, FunctionMetadata,
    ModuleConstantMetadata, StorageEntryMetadata, StorageEntryModifier, StorageEntryType,
    StorageHasher, StorageMetadata, Type,
};

#[derive(Debug, Decode)]
pub struct Metadata {
    pub modules: Vec<ModuleMetadata>,
    pub extrinsic: ExtrinsicMetadata,
}

#[derive(Debug, Decode)]
pub struct ModuleMetadata {
    pub name: String,
    pub storage: Option<StorageMetadata>,
    pub calls: Option<Vec<FunctionMetadata>>,
    pub events: Option<Vec<EventMetadata>>,
    pub constants: Vec<ModuleConstantMetadata>,
    pub errors: Vec<ErrorMetadata>,
    pub index: u8,
}
