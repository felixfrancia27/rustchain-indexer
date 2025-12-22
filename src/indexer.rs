use anyhow::{Context, Result};
use ethers::middleware::Middleware;
use ethers::providers::{Http, Provider};
use ethers::types::{Block, Transaction, U256};
use futures::stream::{self, StreamExt};
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

use crate::config::Config;
use crate::elasticsearch::ElasticsearchClient;
use crate::models::{IndexedBlock, IndexedTransaction};

pub struct BlockIndexer {
    provider: Arc<Provider<Http>>,
    es_client: ElasticsearchClient,
    config: Config,
}

impl BlockIndexer {
    pub async fn new(config: Config) -> Result<Self> {
        info!("Initializing Blockchain Indexer...");
        info!("  RPC URL: {}", config.rpc_url);
        info!("  Elasticsearch URL: {}", config.es_url);
        info!("  Index Prefix: {}", config.index_prefix);
        info!("  Batch Size: {}", config.batch_size);
        info!("  Start Block: {}", config.start_block);
        info!("  Concurrency: {}", config.concurrency);
        info!("  ES Bulk Size: {}", config.es_bulk_size);

        let provider = Arc::new(
            Provider::<Http>::try_from(&config.rpc_url).context("Failed to create RPC provider")?,
        );

        info!("Connected to RPC provider successfully");

        let es_client = ElasticsearchClient::new(&config).await?;

        info!("Connected to Elasticsearch successfully");

        Ok(BlockIndexer {
            provider,
            es_client,
            config,
        })
    }

    pub async fn sync_historical(&self) -> Result<()> {
        info!("");
        info!("========== HISTORICAL SYNC ==========");

        // Get checkpoint from Elasticsearch
        let last_indexed = self.es_client.get_last_indexed_block().await?;
        info!("Last indexed block in Elasticsearch: {}", last_indexed);
        info!("Configured start block: {}", self.config.start_block);

        let start_block = last_indexed.max(self.config.start_block);
        let current_block = self.provider.get_block_number().await?.as_u64();

        info!("Current block on chain: {}", current_block);
        info!("Will start indexing from block: {}", start_block);

        if start_block >= current_block {
            info!("Already synced up to current block!");
            info!("Status: Up to date (block {})", current_block);
            return Ok(());
        }

        let total_blocks = current_block - start_block;
        info!("Total blocks to sync: {}", total_blocks);
        info!("Batch size: {}", self.config.batch_size);
        let estimated_batches = (total_blocks as f64 / self.config.batch_size as f64).ceil() as u64;
        info!("Estimated batches: {}", estimated_batches);
        info!("");

        let batch_size = self.config.batch_size;
        let mut processed = 0;
        let total_to_process = total_blocks;
        let start_time = SystemTime::now();

        for batch_start in (start_block..=current_block).step_by(batch_size) {
            let batch_end = (batch_start + batch_size as u64 - 1).min(current_block);
            let batch_size_actual = batch_end - batch_start + 1;

            info!(
                "Processing batch: blocks {} to {} ({} blocks)",
                batch_start, batch_end, batch_size_actual
            );

            match self.index_block_range(batch_start, batch_end).await {
                Ok(_) => {
                    processed += batch_size_actual as usize;

                    // Calculate progress
                    let processed_u64 = processed as u64;
                    let progress_pct = (processed_u64 as f64 / total_to_process as f64) * 100.0;
                    let elapsed = start_time.elapsed().unwrap().as_secs();
                    let blocks_per_sec = if elapsed > 0 {
                        processed_u64 as f64 / elapsed as f64
                    } else {
                        0.0
                    };
                    let remaining = total_to_process.saturating_sub(processed_u64);
                    let eta_secs = if blocks_per_sec > 0.0 && remaining > 0 {
                        (remaining as f64 / blocks_per_sec) as u64
                    } else {
                        0
                    };

                    info!("Batch completed: blocks {}-{}", batch_start, batch_end);
                    info!(
                        "  Progress: {}/{} blocks ({:.2}%)",
                        processed, total_to_process, progress_pct
                    );
                    info!("  Speed: {:.2} blocks/sec", blocks_per_sec);
                    if eta_secs > 0 {
                        let eta_mins = eta_secs / 60;
                        let eta_secs_remain = eta_secs % 60;
                        info!("  ETA: {}m {}s", eta_mins, eta_secs_remain);
                    }

                    // Set checkpoint after each batch
                    self.es_client.set_checkpoint(batch_end).await?;
                    debug!("Checkpoint saved: block {}", batch_end);
                }
                Err(e) => {
                    error!("Error syncing blocks {}-{}: {}", batch_start, batch_end, e);
                    // Continue with next batch
                }
            }

            info!("");

            // Small delay to avoid overwhelming the RPC (reduced from 100ms)
            sleep(Duration::from_millis(10)).await;
        }

        let total_time = start_time.elapsed().unwrap().as_secs();
        let total_mins = total_time / 60;
        let total_secs = total_time % 60;

        info!("Historical sync completed!");
        info!("Total blocks indexed: {}", processed);
        info!("Total time: {}m {}s", total_mins, total_secs);
        if total_time > 0 {
            info!(
                "Average speed: {:.2} blocks/sec",
                processed as f64 / total_time as f64
            );
        }
        info!("=====================================");
        info!("");

        Ok(())
    }

    pub async fn sync_live(&self) -> Result<()> {
        info!("");
        info!("========== LIVE SYNC MODE ==========");
        info!("Sync interval: {} seconds", self.config.sync_interval_secs);
        info!("Monitoring for new blocks...");
        info!("====================================");
        info!("");

        loop {
            match self.sync_new_blocks().await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Error in live sync: {}", e);
                }
            }

            sleep(Duration::from_secs(self.config.sync_interval_secs)).await;
        }
    }

    async fn sync_new_blocks(&self) -> Result<()> {
        let last_indexed = self.es_client.get_last_indexed_block().await?;
        let current_block = self.provider.get_block_number().await?.as_u64();

        if current_block > last_indexed {
            let new_blocks = current_block - last_indexed;
            info!(
                "Found {} new block(s): {} to {}",
                new_blocks,
                last_indexed + 1,
                current_block
            );

            for block_num in (last_indexed + 1)..=current_block {
                match self.index_block(block_num).await {
                    Ok(_) => {
                        self.es_client.set_checkpoint(block_num).await?;
                        debug!("Indexed block {} and saved checkpoint", block_num);
                    }
                    Err(e) => {
                        error!("Error indexing block {}: {}", block_num, e);
                        // Continue with next block
                    }
                }
            }

            info!("Live sync completed: now at block {}", current_block);
        } else {
            debug!(
                "No new blocks (last indexed: {}, current: {})",
                last_indexed, current_block
            );
        }

        Ok(())
    }

    async fn index_block_range(&self, from: u64, to: u64) -> Result<()> {
        let semaphore = Arc::new(Semaphore::new(self.config.concurrency));
        let mut indexed_blocks = Vec::new();
        let mut error_count = 0;

        // Process blocks in parallel with concurrency limit
        let block_numbers: Vec<u64> = (from..=to).collect();
        let total_blocks = block_numbers.len();

        let results: Vec<(u64, Result<IndexedBlock>)> = stream::iter(block_numbers.iter().cloned())
            .map(|block_num| {
                let provider = Arc::clone(&self.provider);
                let semaphore = Arc::clone(&semaphore);

                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    let result = Self::index_block_internal(block_num, &provider).await;
                    (block_num, result)
                }
            })
            .buffer_unordered(self.config.concurrency)
            .collect()
            .await;

        // Sort by block number to maintain order
        let mut sorted_results: Vec<_> = results.into_iter().collect();
        sorted_results.sort_by_key(|(block_num, _)| *block_num);

        // Collect successful results and prepare for bulk indexing
        for (block_num, result) in sorted_results {
            match result {
                Ok(block) => {
                    indexed_blocks.push(block);
                    if block_num % 100 == 0 || block_num == to {
                        debug!(
                            "Processed block {} (progress: {}/{})",
                            block_num,
                            indexed_blocks.len(),
                            total_blocks
                        );
                    }
                }
                Err(e) => {
                    error_count += 1;
                    error!("Error processing block {}: {}", block_num, e);
                }
            }
        }

        // Bulk index all blocks at once
        if !indexed_blocks.is_empty() {
            // Index in chunks of es_bulk_size
            for chunk in indexed_blocks.chunks(self.config.es_bulk_size) {
                if let Err(e) = self.es_client.bulk_index_blocks(chunk).await {
                    error!("Error bulk indexing blocks: {}", e);
                    // Fallback to individual indexing
                    for block in chunk {
                        if let Err(e) = self.es_client.index_block(block).await {
                            error!("Error indexing block {}: {}", block.number, e);
                        }
                    }
                }
            }
        }

        let success_count = indexed_blocks.len();
        if error_count > 0 {
            warn!(
                "Batch completed with {} errors out of {} blocks",
                error_count,
                success_count + error_count
            );
        }

        Ok(())
    }

    async fn index_block_internal(
        block_number: u64,
        provider: &Arc<Provider<Http>>,
    ) -> Result<IndexedBlock> {
        // OPTIMIZATION: Use get_block_with_txs to get block with full transactions in one RPC call
        // This eliminates N additional get_transaction calls (where N = number of transactions)
        let block_opt: Option<Block<Transaction>> = provider
            .get_block_with_txs(block_number)
            .await
            .context("Failed to fetch block from RPC")?;

        let block = block_opt.context("Block not found")?;

        // Transactions are already included in the block, no need for separate RPC calls
        let transactions: Vec<IndexedTransaction> = block
            .transactions
            .iter()
            .enumerate()
            .map(|(idx, tx)| IndexedTransaction {
                hash: format!("{:?}", tx.hash),
                from: format!("{:?}", tx.from),
                to: tx.to.map(|a| format!("{:?}", a)),
                value: tx.value.to_string(),
                gas: tx.gas.as_u64(),
                gas_price: tx
                    .gas_price
                    .map(|p: U256| p.to_string())
                    .unwrap_or_else(|| "0".to_string()),
                input: hex::encode(tx.input.as_ref()),
                nonce: tx.nonce.as_u64(),
                transaction_index: Some(idx as u64),
            })
            .collect();

        Self::convert_block_from_full(block, transactions).await
    }

    async fn index_block(&self, block_number: u64) -> Result<()> {
        // OPTIMIZATION: Use get_block_with_txs to get block with full transactions in one RPC call
        let block_opt: Option<Block<Transaction>> = self
            .provider
            .get_block_with_txs(block_number)
            .await
            .context("Failed to fetch block from RPC")?;

        let block = block_opt.context("Block not found")?;

        // Transactions are already included in the block
        let transactions: Vec<IndexedTransaction> = block
            .transactions
            .iter()
            .enumerate()
            .map(|(idx, tx)| IndexedTransaction {
                hash: format!("{:?}", tx.hash),
                from: format!("{:?}", tx.from),
                to: tx.to.map(|a| format!("{:?}", a)),
                value: tx.value.to_string(),
                gas: tx.gas.as_u64(),
                gas_price: tx
                    .gas_price
                    .map(|p: U256| p.to_string())
                    .unwrap_or_else(|| "0".to_string()),
                input: hex::encode(tx.input.as_ref()),
                nonce: tx.nonce.as_u64(),
                transaction_index: Some(idx as u64),
            })
            .collect();

        let indexed_block = Self::convert_block_from_full(block, transactions).await?;
        self.es_client.index_block(&indexed_block).await?;

        Ok(())
    }

    async fn convert_block_from_full(
        block: Block<Transaction>,
        transactions: Vec<IndexedTransaction>,
    ) -> Result<IndexedBlock> {
        let indexed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(IndexedBlock {
            number: block.number.unwrap().as_u64(),
            hash: format!("{:?}", block.hash.unwrap()),
            parent_hash: format!("{:?}", block.parent_hash),
            timestamp: block.timestamp.as_u64(),
            gas_limit: block.gas_limit.as_u64(),
            gas_used: block.gas_used.as_u64(),
            miner: block.author.map(|a| format!("{:?}", a)),
            difficulty: block.difficulty.to_string(),
            total_difficulty: block
                .total_difficulty
                .map(|d| d.to_string())
                .unwrap_or_else(|| "0".to_string()),
            size: block.size.map(|s| s.as_u64()).unwrap_or(0),
            transaction_count: transactions.len(),
            transactions,
            uncles: block.uncles.len(),
            indexed_at,
        })
    }
}
