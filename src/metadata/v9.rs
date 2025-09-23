use parity_scale_codec::Decode;

pub type Type = String;

#[derive(Debug, Decode)]
pub struct ErrorMetadata {
    pub name: String,
    pub docs: Vec<String>,
}

#[derive(Debug, Decode)]
pub struct EventMetadata {
    pub name: String,
    pub args: Vec<Type>,
    pub docs: Vec<String>,
}

#[derive(Debug, Decode)]
pub struct FunctionArgumentMetadata {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, Decode)]
pub struct FunctionMetadata {
    pub name: String,
    pub args: Vec<FunctionArgumentMetadata>,
    pub docs: Vec<String>,
}

#[derive(Debug, Decode)]
pub struct Metadata {
    pub modules: Vec<ModuleMetadata>,
}

#[derive(Debug, Decode)]
pub struct ModuleConstantMetadata {
    pub name: String,
    pub type_: Type,
    pub value: Vec<u8>,
    pub docs: Vec<String>,
}

#[derive(Debug, Decode)]
pub struct ModuleMetadata {
    pub name: String,
    pub storage: Option<StorageMetadata>,
    pub calls: Option<Vec<FunctionMetadata>>,
    pub events: Option<Vec<EventMetadata>>,
    pub constants: Vec<ModuleConstantMetadata>,
    pub errors: Vec<ErrorMetadata>,
}

#[derive(Debug, Decode)]
pub struct StorageEntryMetadata {
    pub name: String,
    pub modifier: StorageEntryModifier,
    pub type_: StorageEntryType,
    pub fallback: Vec<u8>,
    pub docs: Vec<String>,
}

#[derive(Debug, Decode)]
pub enum StorageEntryModifier {
    Optional,
    Default,
    Required,
}

#[derive(Debug, Decode)]
pub enum StorageEntryType {
    Plain(Type),
    Map(StorageEntryMap),
    DoubleMap(StorageEntryDoubleMap),
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
pub enum StorageHasher {
    Blake2b128,
    Blake2b256,
    Twox128,
    Twox256,
    Twox64Concat,
}

#[derive(Debug, Decode)]
pub struct StorageMetadata {
    pub prefix: String,
    pub items: Vec<StorageEntryMetadata>,
}
