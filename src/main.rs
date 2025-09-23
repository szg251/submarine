use clap::Parser;
use tracing::{Level, debug, info};

use crate::{
    error::Error,
    metadata::decoder::VersionedMetadata,
    node_rpc::{client::NodeRPC, models::BlockNumber},
};

mod error;
mod metadata;
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

    let queried_block_number = BlockNumber(0.to_string());
    let block_hash = rpc.chain_get_block_hash(&queried_block_number).await?;

    // let block = rpc.chain_get_block(&block_hash).await?;
    info!("Hash of block #{queried_block_number}: {block_hash}");

    let metadata = rpc.state_get_metadata(&block_hash).await?;
    info!("Metadata version: {:?}", metadata.version);

    let decoded = VersionedMetadata::decode(metadata.version, &mut &metadata.data[..])?;

    debug!("{decoded:?}");

    Ok(())
}
