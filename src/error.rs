use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Elasticsearch error: {0}")]
    #[allow(dead_code)]
    Elasticsearch(String),

    #[error("RPC error: {0}")]
    #[allow(dead_code)]
    Rpc(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elasticsearch_error() {
        let error = IndexerError::Elasticsearch("Connection failed".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Elasticsearch error"));
        assert!(error_msg.contains("Connection failed"));
    }

    #[test]
    fn test_rpc_error() {
        let error = IndexerError::Rpc("Timeout".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("RPC error"));
        assert!(error_msg.contains("Timeout"));
    }

    #[test]
    fn test_serialization_error() {
        let error = IndexerError::Serialization("Invalid JSON".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Serialization error"));
        assert!(error_msg.contains("Invalid JSON"));
    }
}
