use clap::Parser;
use frame_metadata::RuntimeMetadataPrefixed;
use parity_scale_codec::Decode;
use scale_info_legacy::ChainTypeRegistry;
use tracing::{Level, debug, info, warn};

use crate::{
    decoder::extrinsic::decode_extrinsic_any,
    error::Error,
    node_rpc::{
        client::NodeRPC,
        models::{BlockNumber, ChainMetadataBytes, ExtrinsicBytes},
    },
};

mod decoder;
mod error;
mod node_rpc;

/// Infinity Query command line interface
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short, global = true)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let collector = tracing_subscriber::fmt()
        .with_max_level(if args.debug {
            Level::DEBUG
        } else {
            Level::INFO
        })
        // build but do not install the subscriber.
        .finish();
    tracing::subscriber::set_global_default(collector)?;

    info!("Starting chain follower.");

    let rpc = NodeRPC::new("ws://37.27.51.25:9944").await?;

    let system_chain = rpc.system_chain().await?;
    info!("System chain: {system_chain}");

    let system_name = rpc.system_name().await?;
    info!("System name: {system_name}");

    let system_version = rpc.system_version().await?;
    info!("System version: {system_version}");

    let header = rpc.chain_get_last_header().await?;
    info!("Latest block number: {}", header.number);

    let finalized_head_hash = rpc.chain_get_finalized_head().await?;
    let finalized_head = rpc.chain_get_header(finalized_head_hash).await?;
    info!("Finalized block number: {}", finalized_head.number);

    let queried_block_number = BlockNumber(1.to_string());
    let block_hash = rpc.chain_get_block_hash(&queried_block_number).await?;

    let mut signed_block = rpc.chain_get_block(&block_hash).await?;
    info!("Hash of block #{queried_block_number}: {block_hash}");

    let runtime_version = rpc.state_get_runtime_version(&block_hash).await?;

    let ChainMetadataBytes(metadata_bytes) = rpc.state_get_metadata(&block_hash).await?;

    let RuntimeMetadataPrefixed(_, metadata) =
        RuntimeMetadataPrefixed::decode(&mut &metadata_bytes[..])
            .map_err(Error::ParsingRuntimeMetadataFailed)?;

    info!("Metadata version: {:?}", metadata.version());

    debug!("{metadata:?}");

    let historic_type_bytes =
        std::fs::read("types/polkadot_types.yaml").map_err(Error::ReadingMetadataFileFailed)?;
    let historic_types: ChainTypeRegistry =
        serde_yaml::from_slice(&historic_type_bytes).map_err(Error::ParsingMetadataFileFailed)?;

    signed_block
        .block
        .extrinsics
        .iter_mut()
        .for_each(|ExtrinsicBytes(ext)| {
            match decode_extrinsic_any(
                &historic_types,
                ext,
                &metadata,
                runtime_version.spec_version,
            ) {
                Ok(extrinsic) => info!("{extrinsic:?}"),
                Err(error) => warn!("{error}"),
            };
        });

    Ok(())
}
