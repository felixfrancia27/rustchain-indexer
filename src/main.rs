//! Blockchain Indexer
//!
//! A Rust service that indexes blockchain blocks and transactions into Elasticsearch.
//! Supports both historical backfill and real-time synchronization.

mod config;
mod elasticsearch;
mod error;
mod indexer;
mod models;

use anyhow::Result;
use config::Config;
use indexer::BlockIndexer;
use log::info;

/// Main entry point for the blockchain indexer
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting Blockchain Indexer...");

    let config = Config::from_env()?;
    let indexer = BlockIndexer::new(config).await?;

    // Run historical sync first
    info!("Starting historical sync...");
    indexer.sync_historical().await?;

    // Then keep syncing live
    info!("Starting live sync...");
    indexer.sync_live().await?;

    Ok(())
}
