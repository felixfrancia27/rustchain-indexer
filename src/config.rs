use anyhow::{Context, Result};
use std::env;

/// Configuration for the blockchain indexer
pub struct Config {
    pub rpc_url: String,
    pub es_url: String,
    pub es_username: Option<String>,
    pub es_password: Option<String>,
    pub index_prefix: String,
    pub batch_size: usize,
    pub start_block: u64,
    pub sync_interval_secs: u64,
    pub concurrency: usize,
    pub es_bulk_size: usize,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(Config {
            rpc_url: env::var("RPC_HTTP_URL")
                .context("RPC_HTTP_URL environment variable is required")?,
            es_url: env::var("ES_URL").context("ES_URL environment variable is required")?,
            es_username: env::var("ES_USERNAME").ok(),
            es_password: env::var("ES_PASSWORD").ok(),
            index_prefix: env::var("INDEX_PREFIX").unwrap_or_else(|_| "workqueue".to_string()),
            batch_size: env::var("BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            start_block: env::var("START_BLOCK")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            sync_interval_secs: env::var("SYNC_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2),
            concurrency: env::var("CONCURRENCY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            es_bulk_size: env::var("ES_BULK_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        })
    }

    /// Get the name of the blocks index
    pub fn blocks_index(&self) -> String {
        format!("{}-blocks", self.index_prefix)
    }

    /// Get the name of the metadata index
    pub fn meta_index(&self) -> String {
        format!("{}-meta", self.index_prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks_index() {
        let config = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: None,
            es_password: None,
            index_prefix: "test".to_string(),
            batch_size: 100,
            start_block: 0,
            sync_interval_secs: 2,
            concurrency: 10,
            es_bulk_size: 100,
        };

        assert_eq!(config.blocks_index(), "test-blocks");
    }

    #[test]
    fn test_meta_index() {
        let config = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: None,
            es_password: None,
            index_prefix: "test".to_string(),
            batch_size: 100,
            start_block: 0,
            sync_interval_secs: 2,
            concurrency: 10,
            es_bulk_size: 100,
        };

        assert_eq!(config.meta_index(), "test-meta");
    }

    #[test]
    fn test_index_names_with_different_prefixes() {
        let config1 = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: None,
            es_password: None,
            index_prefix: "custom".to_string(),
            batch_size: 100,
            start_block: 0,
            sync_interval_secs: 2,
            concurrency: 10,
            es_bulk_size: 100,
        };

        assert_eq!(config1.blocks_index(), "custom-blocks");
        assert_eq!(config1.meta_index(), "custom-meta");

        let config2 = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: None,
            es_password: None,
            index_prefix: "".to_string(),
            batch_size: 100,
            start_block: 0,
            sync_interval_secs: 2,
            concurrency: 10,
            es_bulk_size: 100,
        };

        assert_eq!(config2.blocks_index(), "-blocks");
        assert_eq!(config2.meta_index(), "-meta");
    }

    #[test]
    fn test_config_with_credentials() {
        let config = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: Some("user".to_string()),
            es_password: Some("pass".to_string()),
            index_prefix: "test".to_string(),
            batch_size: 500,
            start_block: 1000,
            sync_interval_secs: 5,
            concurrency: 20,
            es_bulk_size: 200,
        };

        assert!(config.es_username.is_some());
        assert!(config.es_password.is_some());
        assert_eq!(config.batch_size, 500);
        assert_eq!(config.start_block, 1000);
        assert_eq!(config.sync_interval_secs, 5);
        assert_eq!(config.concurrency, 20);
        assert_eq!(config.es_bulk_size, 200);
    }

    #[test]
    fn test_index_names_with_special_characters() {
        let config = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: None,
            es_password: None,
            index_prefix: "test-prefix_123".to_string(),
            batch_size: 100,
            start_block: 0,
            sync_interval_secs: 2,
            concurrency: 10,
            es_bulk_size: 100,
        };

        assert_eq!(config.blocks_index(), "test-prefix_123-blocks");
        assert_eq!(config.meta_index(), "test-prefix_123-meta");
    }

    #[test]
    fn test_config_without_credentials() {
        let config = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: None,
            es_password: None,
            index_prefix: "test".to_string(),
            batch_size: 1000,
            start_block: 0,
            sync_interval_secs: 2,
            concurrency: 10,
            es_bulk_size: 100,
        };

        assert!(config.es_username.is_none());
        assert!(config.es_password.is_none());
        assert_eq!(config.batch_size, 1000); // Default value
    }

    #[test]
    fn test_config_with_partial_credentials() {
        let config1 = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: Some("user".to_string()),
            es_password: None,
            index_prefix: "test".to_string(),
            batch_size: 100,
            start_block: 0,
            sync_interval_secs: 2,
            concurrency: 10,
            es_bulk_size: 100,
        };

        let config2 = Config {
            rpc_url: "http://localhost:8545".to_string(),
            es_url: "http://localhost:9200".to_string(),
            es_username: None,
            es_password: Some("pass".to_string()),
            index_prefix: "test".to_string(),
            batch_size: 100,
            start_block: 0,
            sync_interval_secs: 2,
            concurrency: 10,
            es_bulk_size: 100,
        };

        // Both should have partial credentials
        assert!(config1.es_username.is_some());
        assert!(config1.es_password.is_none());
        assert!(config2.es_username.is_none());
        assert!(config2.es_password.is_some());
    }
}
