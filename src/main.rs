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
    // Force immediate output to stdout/stderr (no buffering)
    use std::io::Write;

    // Escribir directamente a stdout/stderr sin buffering
    println!("=== Blockchain Indexer Starting ===");
    eprintln!("=== Blockchain Indexer Starting (stderr) ===");
    std::io::stdout().flush().ok();
    std::io::stderr().flush().ok();

    // Initialize logger with default level if RUST_LOG is not set
    // Railway necesita logs en stdout sin buffering
    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    println!("RUST_LOG={}", rust_log);
    std::io::stdout().flush().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            use std::io::Write;
            writeln!(buf, "[{}] {}", record.level(), record.args())
        })
        .format_timestamp_secs()
        .target(env_logger::Target::Stdout)
        .init();

    info!("Starting Blockchain Indexer...");
    std::io::stdout().flush().ok();

    if let Err(e) = run().await {
        eprintln!("Fatal error: {:?}", e);
        eprintln!("Error details: {}", e);
        std::io::stderr().flush().ok();
        std::process::exit(1);
    }

    Ok(())
}

async fn run() -> Result<()> {
    use std::io::Write;

    info!("Loading configuration from environment variables...");
    std::io::stdout().flush().ok();

    let config = match Config::from_env() {
        Ok(c) => {
            info!("Configuration loaded successfully");
            std::io::stdout().flush().ok();
            c
        }
        Err(e) => {
            eprintln!("ERROR: Failed to load configuration: {}", e);
            std::io::stderr().flush().ok();
            return Err(e);
        }
    };

    info!("Initializing indexer...");
    std::io::stdout().flush().ok();

    let indexer = match BlockIndexer::new(config).await {
        Ok(i) => {
            info!("Indexer initialized successfully");
            std::io::stdout().flush().ok();
            i
        }
        Err(e) => {
            eprintln!("ERROR: Failed to initialize indexer: {}", e);
            std::io::stderr().flush().ok();
            return Err(e);
        }
    };

    // Run historical sync first
    info!("Starting historical sync...");
    std::io::stdout().flush().ok();

    if let Err(e) = indexer.sync_historical().await {
        eprintln!("ERROR: Historical sync failed: {}", e);
        std::io::stderr().flush().ok();
        return Err(e);
    }

    info!("Historical sync completed");
    std::io::stdout().flush().ok();

    // Then keep syncing live
    info!("Starting live sync...");
    std::io::stdout().flush().ok();

    if let Err(e) = indexer.sync_live().await {
        eprintln!("ERROR: Live sync failed: {}", e);
        std::io::stderr().flush().ok();
        return Err(e);
    }

    Ok(())
}
