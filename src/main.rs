use std::collections::HashSet;

use chrono::{DateTime, Utc};
use clap::Parser;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed, decode_different::DecodeDifferent};
use parity_scale_codec::Decode;
use prettytable::{Table, row, table};
use tracing::{Instrument, Level, debug, info, span, warn};

use crate::{
    decoder::{
        events::{Phase, SYSTEM_EVENTS_KEY, decode_events_any},
        extrinsic::decode_extrinsic_any,
        pallets::ethereum,
        storage::{AnyStorageValue, decode_storage_value_any, encode_storage_key_any},
        value_parser::parse_timestamp,
    },
    error::Error,
    node_rpc::{
        client::NodeRPC,
        models::{
            BlockHashHex, BlockNumberHex, ChainMetadataBytes, ExtrinsicBytes, RuntimeVersion,
            StorageKeyHex, StorageValueBytes,
        },
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

    #[arg(long, short, default_value = "10000")]
    block_number: u32,

    #[arg(long, short, default_value = "ws://37.27.51.25:9944")]
    node_rpc_url: String,
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

    let rpc = NodeRPC::new(&args.node_rpc_url).await?;

    let system_chain = rpc.system_chain().await?;
    info!("System chain: {system_chain}");

    let system_name = rpc.system_name().await?;
    info!("System name: {system_name}");

    let system_version = rpc.system_version().await?;
    info!("System version: {system_version}");

    let sync_state = rpc.system_sync_state().await?;
    info!(
        "Sync state: starting block {}, current block {}, highest block {}",
        sync_state.starting_block, sync_state.current_block, sync_state.highest_block
    );

    fetch_block(&rpc, args.block_number)
        .instrument(span!(
            Level::INFO,
            "Fetch block",
            block_number = args.block_number
        ))
        .await?;

    Ok(())
}

async fn fetch_block(rpc: &NodeRPC, block_number: u32) -> Result<(), Error> {
    let queried_block_number = BlockNumberHex::from(block_number);
    let block_hash = rpc.chain_get_block_hash(&queried_block_number).await?;

    let ChainMetadataBytes(metadata_bytes) = rpc.state_get_metadata(&block_hash).await?;

    let RuntimeMetadataPrefixed(_, metadata) =
        RuntimeMetadataPrefixed::decode(&mut &metadata_bytes[..])
            .map_err(Error::ParsingRuntimeMetadataFailed)?;

    debug!(?metadata);

    let pallets = get_pallets(&metadata)?;

    info!(?pallets);

    let mut signed_block = rpc.chain_get_block(&block_hash).await?;
    debug!(?signed_block);

    let runtime_version = rpc.state_get_runtime_version(&block_hash).await?;

    let timestamp = fetch_timestamp(rpc, &block_hash, &metadata, &runtime_version).await?;

    let mut block_table = table![
        ["Timestamp", timestamp],
        ["Block Time"],
        ["Status"],
        ["Hash", block_hash],
        ["Parent Hash", signed_block.block.header.parent_hash],
        ["State Root", signed_block.block.header.state_root],
        ["Extrinsics Root", signed_block.block.header.extrinsics_root],
        ["Validator"],
        ["Ref Time"],
        ["Spec Version", runtime_version.spec_version]
    ];

    block_table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    println!("Block");
    block_table.printstd();

    let mut extrinsics_table = Table::new();
    extrinsics_table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    signed_block
        .block
        .extrinsics
        .iter_mut()
        .enumerate()
        .for_each(|(i, ExtrinsicBytes(ext))| {
            match decode_extrinsic_any(ext, &metadata, runtime_version.spec_version) {
                Ok(extrinsic) => {
                    let extrinsic_id = format!("{block_number}-{i}");
                    let action = format!("{} ({})", extrinsic.pallet_name(), extrinsic.call_name());
                    extrinsics_table.add_row(row![extrinsic_id, "hash", "time", "result", action]);
                    debug!(?extrinsic);
                }
                Err(error) => warn!("{error}"),
            };
        });

    println!("Extrinsics");
    extrinsics_table.printstd();

    fetch_events(rpc, block_number, &block_hash, &metadata, &runtime_version).await?;

    if pallets.contains("Ethereum") {
        let ethereum_block =
            ethereum::fetch_block(rpc, &block_hash, &metadata, &runtime_version).await?;
        debug!(?ethereum_block);

        let ethereum_block_hash =
            ethereum::fetch_block_hash(rpc, block_number, &block_hash, &metadata, &runtime_version)
                .await?;

        let mut block_table = table![
            ["Status"],
            ["Hash", format!("0x{}", hex::encode(ethereum_block_hash))],
            [
                "Parent Hash",
                format!("0x{}", hex::encode(ethereum_block.header.parent_hash))
            ],
            ["State Root",],
            ["Mined By"]
        ];

        block_table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        println!("Ethereum Block");
        block_table.printstd();
    }

    Ok(())
}

async fn fetch_timestamp(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: &RuntimeMetadata,
    runtime_version: &RuntimeVersion,
) -> Result<DateTime<Utc>, Error> {
    let pallet_name = "Timestamp";
    let storage_entry_name = "Now";

    let timestamp_keys: [u8; 0] = [];
    let timestamp_key = encode_storage_key_any(
        pallet_name,
        storage_entry_name,
        timestamp_keys,
        metadata,
        runtime_version.spec_version,
    )?;

    let timestamp_key_hex = StorageKeyHex::from(timestamp_key.0);

    let StorageValueBytes(storage_bytes) = rpc
        .state_get_storage(&timestamp_key_hex, block_hash)
        .await?
        .ok_or(Error::StorageValueNotFound(timestamp_key_hex.0))?;

    let value = decode_storage_value_any(
        storage_bytes,
        pallet_name,
        storage_entry_name,
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match value {
        AnyStorageValue::Legacy(value) => parse_timestamp(*value)?,
        AnyStorageValue::Modern(value) => parse_timestamp(value)?,
    })
}

async fn fetch_events(
    rpc: &NodeRPC,
    block_number: u32,
    block_hash: &BlockHashHex,
    metadata: &RuntimeMetadata,
    runtime_version: &RuntimeVersion,
) -> Result<(), Error> {
    let system_events_key_hex = StorageKeyHex(SYSTEM_EVENTS_KEY.to_string());

    let mut events_table = Table::new();
    events_table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    if let Some(StorageValueBytes(events_bytes)) = rpc
        .state_get_storage(&system_events_key_hex, block_hash)
        .await?
    {
        decode_events_any(events_bytes, metadata, runtime_version.spec_version)?
            .iter()
            .enumerate()
            .for_each(|(i, event)| {
                let event_id = format!("{block_number}-{i}");
                let (ty, ext_id) = match event.phase {
                    Phase::ApplyExtrinsic(ext_idx) => {
                        ("Extrinsic", Some(format!("{block_number}-{ext_idx}")))
                    }
                    Phase::Initialization => ("Initialization", None),
                    Phase::Finalization => ("Finalization", None),
                };
                let action = format!("{} ({})", event.event.name, event.event.action);
                events_table.add_row(row![
                    event_id,
                    ext_id.unwrap_or(String::from("-")),
                    action,
                    ty
                ]);
                debug!(?event);
            });
    }

    println!("Events");
    events_table.printstd();

    Ok(())
}

fn get_pallets(metadata: &RuntimeMetadata) -> Result<HashSet<String>, Error> {
    match metadata {
        RuntimeMetadata::V8(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V9(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V10(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V11(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V12(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V13(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
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
        _ => Err(Error::UnsupportedMetadataVersion {
            version: metadata.version(),
        }),
    }
}

fn match_decode_different<B, O>(decode_different: &DecodeDifferent<B, O>) -> Result<&O, Error> {
    match decode_different {
        DecodeDifferent::Encode(_) => Err(Error::DecodedDataUnavailable),
        DecodeDifferent::Decoded(decoded) => Ok(decoded),
    }
}
