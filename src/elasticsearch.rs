use crate::config::Config;
use crate::error::IndexerError;
use crate::models::IndexedBlock;
use anyhow::Result;
use elasticsearch::{
    http::transport::Transport,
    indices::{IndicesCreateParts, IndicesExistsParts, IndicesRefreshParts},
    BulkOperation, BulkParts, Elasticsearch, GetParts, IndexParts,
};
use serde_json::{json, Value};

pub struct ElasticsearchClient {
    client: Elasticsearch,
    blocks_index: String,
    meta_index: String,
}

impl ElasticsearchClient {
    pub async fn new(config: &Config) -> Result<Self> {
        let mut url = config.es_url.clone();
        if !url.starts_with("http://") && !url.starts_with("https://") {
            url = format!("http://{}", url);
        }

        // Build URL with credentials if provided
        let final_url =
            if let (Some(username), Some(password)) = (&config.es_username, &config.es_password) {
                let parsed = url::Url::parse(&url)?;
                let mut new_url = parsed.clone();
                new_url
                    .set_username(username)
                    .map_err(|_| anyhow::anyhow!("Invalid username"))?;
                new_url
                    .set_password(Some(password))
                    .map_err(|_| anyhow::anyhow!("Invalid password"))?;
                new_url.to_string()
            } else {
                url
            };

        let transport = Transport::single_node(&final_url)?;
        let client = Elasticsearch::new(transport);

        let es_client = ElasticsearchClient {
            client,
            blocks_index: config.blocks_index(),
            meta_index: config.meta_index(),
        };

        es_client.create_indices().await?;

        Ok(es_client)
    }

    async fn create_indices(&self) -> Result<()> {
        // Create blocks index
        let blocks_mapping = json!({
            "mappings": {
                "properties": {
                    "number": { "type": "long" },
                    "hash": { "type": "keyword" },
                    "parent_hash": { "type": "keyword" },
                    "timestamp": { "type": "long" },
                    "gas_limit": { "type": "long" },
                    "gas_used": { "type": "long" },
                    "miner": { "type": "keyword" },
                    "difficulty": { "type": "keyword" },
                    "total_difficulty": { "type": "keyword" },
                    "size": { "type": "long" },
                    "transactions": {
                        "type": "nested",
                        "properties": {
                            "hash": { "type": "keyword" },
                            "from": { "type": "keyword" },
                            "to": { "type": "keyword" },
                            "value": { "type": "keyword" },
                            "gas": { "type": "long" },
                            "gas_price": { "type": "keyword" },
                            "input": { "type": "text" },
                            "nonce": { "type": "long" },
                            "transaction_index": { "type": "long" }
                        }
                    },
                    "transaction_count": { "type": "integer" },
                    "uncles": { "type": "integer" },
                    "indexed_at": { "type": "long" }
                }
            },
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 0
            }
        });

        let exists = self
            .client
            .indices()
            .exists(IndicesExistsParts::Index(&[&self.blocks_index]))
            .send()
            .await?;

        if !exists.status_code().is_success() {
            self.client
                .indices()
                .create(IndicesCreateParts::Index(&self.blocks_index))
                .body(blocks_mapping)
                .send()
                .await?;
            log::info!("Created index: {}", self.blocks_index);
        }

        // Create meta index for checkpoint
        let meta_mapping = json!({
            "mappings": {
                "properties": {
                    "last_indexed_block": { "type": "long" },
                    "updated_at": { "type": "long" }
                }
            }
        });

        let exists = self
            .client
            .indices()
            .exists(IndicesExistsParts::Index(&[&self.meta_index]))
            .send()
            .await?;

        if !exists.status_code().is_success() {
            self.client
                .indices()
                .create(IndicesCreateParts::Index(&self.meta_index))
                .body(meta_mapping)
                .send()
                .await?;
            log::info!("Created index: {}", self.meta_index);
        }

        Ok(())
    }

    pub async fn index_block(&self, block: &IndexedBlock) -> Result<()> {
        let body =
            serde_json::to_string(block).map_err(|e| IndexerError::Serialization(e.to_string()))?;

        self.client
            .index(IndexParts::IndexId(
                &self.blocks_index,
                &block.number.to_string(),
            ))
            .body(body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn bulk_index_blocks(&self, blocks: &[IndexedBlock]) -> Result<()> {
        if blocks.is_empty() {
            return Ok(());
        }

        let mut ops: Vec<BulkOperation<IndexedBlock>> = Vec::with_capacity(blocks.len());

        for block in blocks {
            ops.push(
                BulkOperation::index(block.clone())
                    .id(block.number.to_string())
                    .into(),
            );
        }

        self.client
            .bulk(BulkParts::Index(&self.blocks_index))
            .body(ops)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_last_indexed_block(&self) -> Result<u64> {
        let response = self
            .client
            .get(GetParts::IndexId(&self.meta_index, "checkpoint"))
            .send()
            .await;

        match response {
            Ok(res) => {
                let body: Value = res.json().await?;
                let block = body["_source"]["last_indexed_block"].as_u64().unwrap_or(0);
                log::debug!("Retrieved checkpoint from Elasticsearch: block {}", block);
                Ok(block)
            }
            Err(_) => {
                log::info!("No checkpoint found in Elasticsearch, starting from block 0");
                Ok(0)
            }
        }
    }

    pub async fn set_checkpoint(&self, block_number: u64) -> Result<()> {
        let body = json!({
            "last_indexed_block": block_number,
            "updated_at": chrono::Utc::now().timestamp_millis()
        });

        self.client
            .index(IndexParts::IndexId(&self.meta_index, "checkpoint"))
            .body(body)
            .send()
            .await?;

        log::debug!("Checkpoint saved to Elasticsearch: block {}", block_number);
        Ok(())
    }

    pub async fn refresh_blocks_index(&self) -> Result<()> {
        self.client
            .indices()
            .refresh(IndicesRefreshParts::Index(&[&self.blocks_index]))
            .send()
            .await?;
        Ok(())
    }
}
