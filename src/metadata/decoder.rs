use parity_scale_codec::Decode;
use thiserror::Error;

use crate::metadata::{v9, v10, v11, v12, v13};

#[derive(Debug)]
pub enum VersionedMetadata {
    V9(v9::Metadata),
    V10(v10::Metadata),
    V11(v11::Metadata),
    V12(v12::Metadata),
    V13(v13::Metadata),
}

impl VersionedMetadata {
    pub fn decode(
        version: u8,
        data: &mut impl parity_scale_codec::Input,
    ) -> Result<Self, MetadataDecoderError> {
        match version {
            9 => Ok(Self::V9(v9::Metadata::decode(data)?)),
            10 => Ok(Self::V10(v10::Metadata::decode(data)?)),
            11 => Ok(Self::V11(v11::Metadata::decode(data)?)),
            12 => Ok(Self::V12(v12::Metadata::decode(data)?)),
            13 => Ok(Self::V13(v13::Metadata::decode(data)?)),

            _ => Err(MetadataDecoderError::UnsupportedMetadataVersion { version }),
        }
    }
}

#[derive(Debug, Error)]
pub enum MetadataDecoderError {
    #[error("Unsupported metadata version: {version}")]
    UnsupportedMetadataVersion { version: u8 },

    #[error(transparent)]
    ScaleDecoderFailed(#[from] parity_scale_codec::Error),
}
