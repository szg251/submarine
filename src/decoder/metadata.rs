use std::collections::HashSet;

use frame_metadata::{RuntimeMetadata, decode_different::DecodeDifferent};
use scale_info::PortableRegistry;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("Decoded data unavailable in type DecodeDifferent")]
    DecodedDataUnavailable,

    #[error("Could not found pallet metadata")]
    MetadataNotFound(String),

    #[error("This metadata version is unsupported: {version}")]
    UnsupportedMetadataVersion { version: u32 },

    #[error("Couldn't resolve metadata type: {0}")]
    UnresolvableType(#[from] UnresolvableTypeError),

    #[error(transparent)]
    CantBuldTypeRegistry(#[from] scale_info_legacy::lookup_name::ParseError),

    #[error(
        "Expected a storage value of {expected} but got {got} for {pallet_name} {storage_entry_name}"
    )]
    UnexpectedStorageValueType {
        expected: String,
        got: String,
        pallet_name: String,
        storage_entry_name: String,
    },

    #[error(
        "Expected a storage key of {expected} but got {got} for {pallet_name} {storage_entry_name}"
    )]
    UnexpectedStorageKeyType {
        expected: String,
        got: String,
        pallet_name: String,
        storage_entry_name: String,
    },
}

#[derive(Debug, Error)]
pub enum UnresolvableTypeError {
    #[error("Couldn't find porbale type registry in metadata")]
    RegistryNotFound,

    #[error("Type resolver cannot find type id {0}")]
    TypeIdNotFound(u32),
}

pub trait UnwrapDecodeDifferent<O> {
    fn unwrap_decode_different(&self) -> Result<&O, MetadataError>;
}

impl<B, O> UnwrapDecodeDifferent<O> for DecodeDifferent<B, O> {
    fn unwrap_decode_different(&self) -> Result<&O, MetadataError> {
        match self {
            DecodeDifferent::Encode(_) => Err(MetadataError::DecodedDataUnavailable),
            DecodeDifferent::Decoded(decoded) => Ok(decoded),
        }
    }
}

pub enum AnyPalletMetadata<'a> {
    V8(&'a frame_metadata::v8::ModuleMetadata),
    V9(&'a frame_metadata::v9::ModuleMetadata),
    V10(&'a frame_metadata::v10::ModuleMetadata),
    V11(&'a frame_metadata::v11::ModuleMetadata),
    V12(&'a frame_metadata::v12::ModuleMetadata),
    V13(&'a frame_metadata::v13::ModuleMetadata),
    V14(&'a frame_metadata::v14::PalletMetadata<scale_info::form::PortableForm>),
    V15(&'a frame_metadata::v15::PalletMetadata<scale_info::form::PortableForm>),
    V16(&'a frame_metadata::v16::PalletMetadata<scale_info::form::PortableForm>),
}

impl<'a> AnyPalletMetadata<'a> {
    pub fn version(&self) -> u32 {
        match self {
            Self::V8(_) => 8,
            Self::V9(_) => 9,
            Self::V10(_) => 10,
            Self::V11(_) => 11,
            Self::V12(_) => 12,
            Self::V13(_) => 13,
            Self::V14(_) => 14,
            Self::V15(_) => 15,
            Self::V16(_) => 16,
        }
    }

    pub fn storage_entries(&self) -> Result<Vec<AnyStorageEntry>, MetadataError> {
        match self {
            Self::V8(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .unwrap_decode_different()?
                .entries
                .unwrap_decode_different()?
                .iter()
                .map(AnyStorageEntry::V8)
                .collect()),
            Self::V9(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .unwrap_decode_different()?
                .entries
                .unwrap_decode_different()?
                .iter()
                .map(AnyStorageEntry::V9)
                .collect()),
            Self::V10(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .unwrap_decode_different()?
                .entries
                .unwrap_decode_different()?
                .iter()
                .map(AnyStorageEntry::V10)
                .collect()),
            Self::V11(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .unwrap_decode_different()?
                .entries
                .unwrap_decode_different()?
                .iter()
                .map(AnyStorageEntry::V11)
                .collect()),
            Self::V12(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .unwrap_decode_different()?
                .entries
                .unwrap_decode_different()?
                .iter()
                .map(AnyStorageEntry::V12)
                .collect()),
            Self::V13(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .unwrap_decode_different()?
                .entries
                .unwrap_decode_different()?
                .iter()
                .map(AnyStorageEntry::V13)
                .collect()),
            Self::V14(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .entries
                .iter()
                .map(AnyStorageEntry::V14)
                .collect()),
            Self::V15(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .entries
                .iter()
                .map(AnyStorageEntry::V15)
                .collect()),
            Self::V16(metadata) => Ok(metadata
                .storage
                .as_ref()
                .ok_or(MetadataError::MetadataNotFound("storage".to_string()))?
                .entries
                .iter()
                .map(AnyStorageEntry::V16)
                .collect()),
        }
    }

    pub fn storage_entry<'b>(&self, entry_name: &'b str) -> Result<AnyStorageEntry, MetadataError> {
        let x = self.storage_entries()?.into_iter();

        x.try_find(|entry| Ok::<bool, MetadataError>(entry.name()? == entry_name))?
            .ok_or(MetadataError::MetadataNotFound(entry_name.to_string()))
    }
}

pub enum AnyStorageEntry<'a> {
    V8(&'a frame_metadata::v8::StorageEntryMetadata),
    V9(&'a frame_metadata::v9::StorageEntryMetadata),
    V10(&'a frame_metadata::v10::StorageEntryMetadata),
    V11(&'a frame_metadata::v11::StorageEntryMetadata),
    V12(&'a frame_metadata::v12::StorageEntryMetadata),
    V13(&'a frame_metadata::v13::StorageEntryMetadata),
    V14(&'a frame_metadata::v14::StorageEntryMetadata<scale_info::form::PortableForm>),
    V15(&'a frame_metadata::v15::StorageEntryMetadata<scale_info::form::PortableForm>),
    V16(&'a frame_metadata::v16::StorageEntryMetadata<scale_info::form::PortableForm>),
}

impl<'a> AnyStorageEntry<'a> {
    pub fn version(&self) -> u32 {
        match self {
            Self::V8(_) => 8,
            Self::V9(_) => 9,
            Self::V10(_) => 10,
            Self::V11(_) => 11,
            Self::V12(_) => 12,
            Self::V13(_) => 13,
            Self::V14(_) => 14,
            Self::V15(_) => 15,
            Self::V16(_) => 16,
        }
    }

    pub fn name(&self) -> Result<&str, MetadataError> {
        match self {
            Self::V8(entry) => Ok(&entry.name.unwrap_decode_different()?[..]),
            Self::V9(entry) => Ok(&entry.name.unwrap_decode_different()?[..]),
            Self::V10(entry) => Ok(&entry.name.unwrap_decode_different()?[..]),
            Self::V11(entry) => Ok(&entry.name.unwrap_decode_different()?[..]),
            Self::V12(entry) => Ok(&entry.name.unwrap_decode_different()?[..]),
            Self::V13(entry) => Ok(&entry.name.unwrap_decode_different()?[..]),
            Self::V14(entry) => Ok(&entry.name[..]),
            Self::V15(entry) => Ok(&entry.name[..]),
            Self::V16(entry) => Ok(&entry.name[..]),
        }
    }

    /// Returns the key and value types as a tuple
    /// DoubleMap and NMap contain multiple keys hence the use of a Vector
    pub fn types_as_str(
        &self,
        type_registry: Option<&PortableRegistry>,
    ) -> Result<(Vec<String>, String), MetadataError> {
        match self {
            Self::V8(entry) => match entry.ty {
                frame_metadata::v8::StorageEntryType::Plain(ref value) => {
                    Ok((vec![], value.unwrap_decode_different()?.to_owned()))
                }
                frame_metadata::v8::StorageEntryType::Map {
                    ref key, ref value, ..
                } => Ok((
                    vec![key.unwrap_decode_different()?.to_owned()],
                    value.unwrap_decode_different()?.to_owned(),
                )),
                frame_metadata::v8::StorageEntryType::DoubleMap {
                    ref key1,
                    ref key2,
                    ref value,
                    ..
                } => Ok((
                    vec![
                        key1.unwrap_decode_different()?.to_owned(),
                        key2.unwrap_decode_different()?.to_owned(),
                    ],
                    value.unwrap_decode_different()?.to_owned(),
                )),
            },
            Self::V9(entry) => match entry.ty {
                frame_metadata::v9::StorageEntryType::Plain(ref value) => {
                    Ok((vec![], value.unwrap_decode_different()?.to_owned()))
                }
                frame_metadata::v9::StorageEntryType::Map {
                    ref key, ref value, ..
                } => Ok((
                    vec![key.unwrap_decode_different()?.to_owned()],
                    value.unwrap_decode_different()?.to_owned(),
                )),
                frame_metadata::v9::StorageEntryType::DoubleMap {
                    ref key1,
                    ref key2,
                    ref value,
                    ..
                } => Ok((
                    vec![
                        key1.unwrap_decode_different()?.to_owned(),
                        key2.unwrap_decode_different()?.to_owned(),
                    ],
                    value.unwrap_decode_different()?.to_owned(),
                )),
            },
            Self::V10(entry) => match entry.ty {
                frame_metadata::v10::StorageEntryType::Plain(ref value) => {
                    Ok((vec![], value.unwrap_decode_different()?.to_owned()))
                }
                frame_metadata::v10::StorageEntryType::Map {
                    ref key, ref value, ..
                } => Ok((
                    vec![key.unwrap_decode_different()?.to_owned()],
                    value.unwrap_decode_different()?.to_owned(),
                )),
                frame_metadata::v10::StorageEntryType::DoubleMap {
                    ref key1,
                    ref key2,
                    ref value,
                    ..
                } => Ok((
                    vec![
                        key1.unwrap_decode_different()?.to_owned(),
                        key2.unwrap_decode_different()?.to_owned(),
                    ],
                    value.unwrap_decode_different()?.to_owned(),
                )),
            },
            Self::V11(entry) => match entry.ty {
                frame_metadata::v11::StorageEntryType::Plain(ref value) => {
                    Ok((vec![], value.unwrap_decode_different()?.to_owned()))
                }
                frame_metadata::v11::StorageEntryType::Map {
                    ref key, ref value, ..
                } => Ok((
                    vec![key.unwrap_decode_different()?.to_owned()],
                    value.unwrap_decode_different()?.to_owned(),
                )),
                frame_metadata::v11::StorageEntryType::DoubleMap {
                    ref key1,
                    ref key2,
                    ref value,
                    ..
                } => Ok((
                    vec![
                        key1.unwrap_decode_different()?.to_owned(),
                        key2.unwrap_decode_different()?.to_owned(),
                    ],
                    value.unwrap_decode_different()?.to_owned(),
                )),
            },
            Self::V12(entry) => match entry.ty {
                frame_metadata::v12::StorageEntryType::Plain(ref value) => {
                    Ok((vec![], value.unwrap_decode_different()?.to_owned()))
                }
                frame_metadata::v12::StorageEntryType::Map {
                    ref key, ref value, ..
                } => Ok((
                    vec![key.unwrap_decode_different()?.to_owned()],
                    value.unwrap_decode_different()?.to_owned(),
                )),
                frame_metadata::v12::StorageEntryType::DoubleMap {
                    ref key1,
                    ref key2,
                    ref value,
                    ..
                } => Ok((
                    vec![
                        key1.unwrap_decode_different()?.to_owned(),
                        key2.unwrap_decode_different()?.to_owned(),
                    ],
                    value.unwrap_decode_different()?.to_owned(),
                )),
            },
            Self::V13(entry) => match entry.ty {
                frame_metadata::v13::StorageEntryType::Plain(ref value) => {
                    Ok((vec![], value.unwrap_decode_different()?.to_owned()))
                }
                frame_metadata::v13::StorageEntryType::Map {
                    ref key, ref value, ..
                } => Ok((
                    vec![key.unwrap_decode_different()?.to_owned()],
                    value.unwrap_decode_different()?.to_owned(),
                )),
                frame_metadata::v13::StorageEntryType::DoubleMap {
                    ref key1,
                    ref key2,
                    ref value,
                    ..
                } => Ok((
                    vec![
                        key1.unwrap_decode_different()?.to_owned(),
                        key2.unwrap_decode_different()?.to_owned(),
                    ],
                    value.unwrap_decode_different()?.to_owned(),
                )),
                frame_metadata::v13::StorageEntryType::NMap {
                    ref keys,
                    ref value,
                    ..
                } => Ok((
                    keys.unwrap_decode_different()?.to_owned(),
                    value.unwrap_decode_different()?.to_owned(),
                )),
            },
            Self::V14(entry) => {
                let type_registry = type_registry.ok_or(UnresolvableTypeError::RegistryNotFound)?;

                match entry.ty {
                    frame_metadata::v14::StorageEntryType::Plain(value) => {
                        Ok((vec![], resolve_type_to_str(value.id, type_registry)?))
                    }
                    frame_metadata::v14::StorageEntryType::Map { key, value, .. } => Ok((
                        vec![resolve_type_to_str(key.id, type_registry)?],
                        resolve_type_to_str(value.id, type_registry)?,
                    )),
                }
            }
            Self::V15(entry) => {
                let type_registry = type_registry.ok_or(UnresolvableTypeError::RegistryNotFound)?;

                match entry.ty {
                    frame_metadata::v14::StorageEntryType::Plain(value) => {
                        Ok((vec![], resolve_type_to_str(value.id, type_registry)?))
                    }
                    frame_metadata::v14::StorageEntryType::Map { key, value, .. } => Ok((
                        vec![resolve_type_to_str(key.id, type_registry)?],
                        resolve_type_to_str(value.id, type_registry)?,
                    )),
                }
            }
            Self::V16(entry) => {
                let type_registry = type_registry.ok_or(UnresolvableTypeError::RegistryNotFound)?;

                match entry.ty {
                    frame_metadata::v14::StorageEntryType::Plain(value) => {
                        Ok((vec![], resolve_type_to_str(value.id, type_registry)?))
                    }
                    frame_metadata::v14::StorageEntryType::Map { key, value, .. } => Ok((
                        vec![resolve_type_to_str(key.id, type_registry)?],
                        resolve_type_to_str(value.id, type_registry)?,
                    )),
                }
            }
        }
    }
}

fn resolve_type_to_str(
    type_id: u32,
    type_registry: &PortableRegistry,
) -> Result<String, UnresolvableTypeError> {
    let resolved_ty = type_registry
        .resolve(type_id)
        .ok_or(UnresolvableTypeError::TypeIdNotFound(type_id))?;

    let ty_params = resolved_ty
        .type_params
        .iter()
        .filter_map(|ty_param| {
            ty_param.ty.map(|ty_id| {
                let meta = type_registry
                    .resolve(ty_id.id)
                    .ok_or(UnresolvableTypeError::TypeIdNotFound(ty_id.id))?;

                Ok(meta.path.segments.join("::"))
            })
        })
        .collect::<Result<Vec<String>, UnresolvableTypeError>>()?;

    let mut ty_as_str = resolved_ty.path.segments.join("::");
    if !ty_params.is_empty() {
        ty_as_str.push('<');
        ty_as_str.push_str(&ty_params.join(", "));
        ty_as_str.push('>');
    }
    Ok(ty_as_str)
}

#[derive(Debug, Clone, Copy)]
pub struct AnyRuntimeMetadata<'a>(pub &'a RuntimeMetadata);

impl<'a> AnyRuntimeMetadata<'a> {
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    pub fn pallet_names(&self) -> Result<HashSet<String>, MetadataError> {
        match self.0 {
            RuntimeMetadata::V8(metadata) => metadata
                .modules
                .unwrap_decode_different()?
                .iter()
                .map(|module| module.name.unwrap_decode_different().cloned())
                .collect(),
            RuntimeMetadata::V9(metadata) => metadata
                .modules
                .unwrap_decode_different()?
                .iter()
                .map(|module| module.name.unwrap_decode_different().cloned())
                .collect(),
            RuntimeMetadata::V10(metadata) => metadata
                .modules
                .unwrap_decode_different()?
                .iter()
                .map(|module| module.name.unwrap_decode_different().cloned())
                .collect(),
            RuntimeMetadata::V11(metadata) => metadata
                .modules
                .unwrap_decode_different()?
                .iter()
                .map(|module| module.name.unwrap_decode_different().cloned())
                .collect(),
            RuntimeMetadata::V12(metadata) => metadata
                .modules
                .unwrap_decode_different()?
                .iter()
                .map(|module| module.name.unwrap_decode_different().cloned())
                .collect(),
            RuntimeMetadata::V13(metadata) => metadata
                .modules
                .unwrap_decode_different()?
                .iter()
                .map(|module| module.name.unwrap_decode_different().cloned())
                .collect(),
            RuntimeMetadata::V14(metadata) => Ok(metadata
                .pallets
                .iter()
                .map(|pallet| pallet.name.clone())
                .collect()),
            RuntimeMetadata::V15(metadata) => Ok(metadata
                .pallets
                .iter()
                .map(|pallet| pallet.name.clone())
                .collect()),
            RuntimeMetadata::V16(metadata) => Ok(metadata
                .pallets
                .iter()
                .map(|pallet| pallet.name.clone())
                .collect()),
            _ => Err(MetadataError::UnsupportedMetadataVersion {
                version: self.version(),
            }),
        }
    }

    pub fn pallet_metadata(
        &self,
        pallet_name: &str,
    ) -> Result<AnyPalletMetadata<'a>, MetadataError> {
        match self.0 {
            RuntimeMetadata::V8(metadata) => Ok(AnyPalletMetadata::V8(
                metadata
                    .modules
                    .unwrap_decode_different()?
                    .iter()
                    .try_find(|module| {
                        Ok::<bool, MetadataError>(
                            module.name.unwrap_decode_different()? == pallet_name,
                        )
                    })?
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            RuntimeMetadata::V9(metadata) => Ok(AnyPalletMetadata::V9(
                metadata
                    .modules
                    .unwrap_decode_different()?
                    .iter()
                    .try_find(|module| {
                        Ok::<bool, MetadataError>(
                            module.name.unwrap_decode_different()? == pallet_name,
                        )
                    })?
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            RuntimeMetadata::V10(metadata) => Ok(AnyPalletMetadata::V10(
                metadata
                    .modules
                    .unwrap_decode_different()?
                    .iter()
                    .try_find(|module| {
                        Ok::<bool, MetadataError>(
                            module.name.unwrap_decode_different()? == pallet_name,
                        )
                    })?
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            RuntimeMetadata::V11(metadata) => Ok(AnyPalletMetadata::V11(
                metadata
                    .modules
                    .unwrap_decode_different()?
                    .iter()
                    .try_find(|module| {
                        Ok::<bool, MetadataError>(
                            module.name.unwrap_decode_different()? == pallet_name,
                        )
                    })?
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            RuntimeMetadata::V12(metadata) => Ok(AnyPalletMetadata::V12(
                metadata
                    .modules
                    .unwrap_decode_different()?
                    .iter()
                    .try_find(|module| {
                        Ok::<bool, MetadataError>(
                            module.name.unwrap_decode_different()? == pallet_name,
                        )
                    })?
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            RuntimeMetadata::V13(metadata) => Ok(AnyPalletMetadata::V13(
                metadata
                    .modules
                    .unwrap_decode_different()?
                    .iter()
                    .try_find(|module| {
                        Ok::<bool, MetadataError>(
                            module.name.unwrap_decode_different()? == pallet_name,
                        )
                    })?
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            RuntimeMetadata::V14(metadata) => Ok(AnyPalletMetadata::V14(
                metadata
                    .pallets
                    .iter()
                    .find(|pallet| pallet.name == pallet_name)
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            RuntimeMetadata::V15(metadata) => Ok(AnyPalletMetadata::V15(
                metadata
                    .pallets
                    .iter()
                    .find(|pallet| pallet.name == pallet_name)
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            RuntimeMetadata::V16(metadata) => Ok(AnyPalletMetadata::V16(
                metadata
                    .pallets
                    .iter()
                    .find(|pallet| pallet.name == pallet_name)
                    .ok_or(MetadataError::MetadataNotFound(pallet_name.to_string()))?,
            )),
            _ => Err(MetadataError::UnsupportedMetadataVersion {
                version: self.version(),
            }),
        }
    }

    pub fn type_registry(&self) -> Option<&'a PortableRegistry> {
        match self.0 {
            RuntimeMetadata::V14(metadata) => Some(&metadata.types),
            RuntimeMetadata::V15(metadata) => Some(&metadata.types),
            RuntimeMetadata::V16(metadata) => Some(&metadata.types),
            _ => None,
        }
    }
}

// TODO(szg251): `try_find` is implemented in nightly, use that once it's in stable
trait TryFind<P, E> {
    fn try_find(self, predicate: P) -> Result<Option<Self::Item>, E>
    where
        Self: Iterator,
        P: Fn(&Self::Item) -> Result<bool, E>;
}

impl<'a, P, T, E> TryFind<P, E> for std::slice::Iter<'a, T> {
    fn try_find(self, predicate: P) -> Result<Option<<Self as Iterator>::Item>, E>
    where
        Self: Iterator,
        P: Fn(&<Self as Iterator>::Item) -> Result<bool, E>,
    {
        let mut result = None;

        for item in self {
            if predicate(&item)? {
                result = Some(item);
                break;
            }
        }

        Ok(result)
    }
}

impl<P, T, E> TryFind<P, E> for std::vec::IntoIter<T> {
    fn try_find(self, predicate: P) -> Result<Option<<Self as Iterator>::Item>, E>
    where
        Self: Iterator,
        P: Fn(&<Self as Iterator>::Item) -> Result<bool, E>,
    {
        let mut result = None;

        for item in self {
            if predicate(&item)? {
                result = Some(item);
                break;
            }
        }

        Ok(result)
    }
}

pub fn verify_pallet_metadata<'a>(
    pallet_name: &str,
    storage_types: impl AsRef<[(&'a str, (&'a [&'a str], &'a str))]>,
    metadata: AnyRuntimeMetadata<'_>,
) -> Result<(), MetadataError> {
    let type_registry = metadata.type_registry();

    storage_types.as_ref().iter().try_for_each(
        |(storage_entry_name, (expected_key_types, expected_value_type))| {
            let (key_types, value_type) = metadata
                .pallet_metadata(pallet_name)?
                .storage_entry(storage_entry_name)?
                .types_as_str(type_registry)?;

            if &value_type[..] != *expected_value_type {
                Err(MetadataError::UnexpectedStorageValueType {
                    expected: expected_value_type.to_string(),
                    got: value_type,
                    pallet_name: pallet_name.to_string(),
                    storage_entry_name: storage_entry_name.to_string(),
                })?
            };

            key_types.iter().enumerate().try_for_each(|(i, key_type)| {
                let expected_key_type = expected_key_types.get(i).copied();
                if Some(&key_type[..]) != expected_key_type {
                    Err(MetadataError::UnexpectedStorageKeyType {
                        expected: format!("{expected_key_type:?}"),
                        got: key_type.to_string(),
                        pallet_name: pallet_name.to_string(),
                        storage_entry_name: storage_entry_name.to_string(),
                    })
                } else {
                    Ok(())
                }
            })
        },
    )
}
