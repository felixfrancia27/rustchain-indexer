use serde::{Deserialize, Serialize};

/// Represents a blockchain block indexed in Elasticsearch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedBlock {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub miner: Option<String>,
    pub difficulty: String,
    pub total_difficulty: String,
    pub size: u64,
    pub transactions: Vec<IndexedTransaction>,
    pub transaction_count: usize,
    pub uncles: usize,
    pub indexed_at: u64,
}

/// Represents a blockchain transaction within an indexed block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedTransaction {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub gas: u64,
    pub gas_price: String,
    pub input: String,
    pub nonce: u64,
    pub transaction_index: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_block_serialization() {
        let block = IndexedBlock {
            number: 1,
            hash: "0x123".to_string(),
            parent_hash: "0x456".to_string(),
            timestamp: 1000,
            gas_limit: 1000000,
            gas_used: 500000,
            miner: Some("0x789".to_string()),
            difficulty: "1000".to_string(),
            total_difficulty: "2000".to_string(),
            size: 100,
            transactions: vec![],
            transaction_count: 0,
            uncles: 0,
            indexed_at: 1234567890,
        };

        let json = serde_json::to_string(&block).unwrap();
        let deserialized: IndexedBlock = serde_json::from_str(&json).unwrap();

        assert_eq!(block.number, deserialized.number);
        assert_eq!(block.hash, deserialized.hash);
    }

    #[test]
    fn test_indexed_transaction_serialization() {
        let tx = IndexedTransaction {
            hash: "0xabc".to_string(),
            from: "0xdef".to_string(),
            to: Some("0xghi".to_string()),
            value: "1000".to_string(),
            gas: 21000,
            gas_price: "20".to_string(),
            input: "0x".to_string(),
            nonce: 0,
            transaction_index: Some(0),
        };

        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: IndexedTransaction = serde_json::from_str(&json).unwrap();

        assert_eq!(tx.hash, deserialized.hash);
        assert_eq!(tx.from, deserialized.from);
    }

    #[test]
    fn test_indexed_block_with_transactions() {
        let transactions = vec![
            IndexedTransaction {
                hash: "0x111".to_string(),
                from: "0xaaa".to_string(),
                to: Some("0xbbb".to_string()),
                value: "100".to_string(),
                gas: 21000,
                gas_price: "20".to_string(),
                input: "0x".to_string(),
                nonce: 0,
                transaction_index: Some(0),
            },
            IndexedTransaction {
                hash: "0x222".to_string(),
                from: "0xccc".to_string(),
                to: None,
                value: "200".to_string(),
                gas: 30000,
                gas_price: "30".to_string(),
                input: "0x1234".to_string(),
                nonce: 1,
                transaction_index: Some(1),
            },
        ];

        let block = IndexedBlock {
            number: 100,
            hash: "0xblock".to_string(),
            parent_hash: "0xparent".to_string(),
            timestamp: 1234567890,
            gas_limit: 2000000,
            gas_used: 1500000,
            miner: None,
            difficulty: "5000".to_string(),
            total_difficulty: "10000".to_string(),
            size: 500,
            transactions: transactions.clone(),
            transaction_count: 2,
            uncles: 0,
            indexed_at: 1234567890,
        };

        assert_eq!(block.transaction_count, block.transactions.len());
        assert_eq!(block.transactions.len(), 2);
        assert_eq!(block.transactions[0].hash, "0x111");
        assert_eq!(block.transactions[1].to, None);
    }

    #[test]
    fn test_indexed_transaction_with_none_to() {
        let tx = IndexedTransaction {
            hash: "0xcontract".to_string(),
            from: "0xdeployer".to_string(),
            to: None, // Contract creation
            value: "0".to_string(),
            gas: 500000,
            gas_price: "100".to_string(),
            input: "0x6080604052".to_string(),
            nonce: 5,
            transaction_index: Some(10),
        };

        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: IndexedTransaction = serde_json::from_str(&json).unwrap();

        assert_eq!(tx.to, deserialized.to);
        assert_eq!(tx.to, None);
    }

    #[test]
    fn test_indexed_block_without_miner() {
        let block = IndexedBlock {
            number: 0,
            hash: "0xgenesis".to_string(),
            parent_hash: "0x0000".to_string(),
            timestamp: 0,
            gas_limit: 5000,
            gas_used: 0,
            miner: None,
            difficulty: "0".to_string(),
            total_difficulty: "0".to_string(),
            size: 0,
            transactions: vec![],
            transaction_count: 0,
            uncles: 0,
            indexed_at: 0,
        };

        let json = serde_json::to_string(&block).unwrap();
        let deserialized: IndexedBlock = serde_json::from_str(&json).unwrap();

        assert_eq!(block.miner, deserialized.miner);
        assert_eq!(block.miner, None);
    }

    #[test]
    fn test_indexed_transaction_without_index() {
        let tx = IndexedTransaction {
            hash: "0xnoindex".to_string(),
            from: "0xfrom".to_string(),
            to: Some("0xto".to_string()),
            value: "0".to_string(),
            gas: 0,
            gas_price: "0".to_string(),
            input: "0x".to_string(),
            nonce: 0,
            transaction_index: None,
        };

        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: IndexedTransaction = serde_json::from_str(&json).unwrap();

        assert_eq!(tx.transaction_index, deserialized.transaction_index);
        assert_eq!(tx.transaction_index, None);
    }

    #[test]
    fn test_indexed_block_with_extreme_values() {
        let block = IndexedBlock {
            number: u64::MAX,
            hash: "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string(),
            parent_hash: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            timestamp: u64::MAX,
            gas_limit: u64::MAX,
            gas_used: u64::MAX,
            miner: Some("0x0000000000000000000000000000000000000000".to_string()),
            difficulty: "999999999999999999999999999999999999999999999999999999999999999999999".to_string(),
            total_difficulty: "999999999999999999999999999999999999999999999999999999999999999999999".to_string(),
            size: u64::MAX,
            transactions: vec![],
            transaction_count: 0,
            uncles: u64::MAX as usize,
            indexed_at: u64::MAX,
        };

        let json = serde_json::to_string(&block).unwrap();
        let deserialized: IndexedBlock = serde_json::from_str(&json).unwrap();

        assert_eq!(block.number, deserialized.number);
        assert_eq!(block.hash, deserialized.hash);
        assert_eq!(block.timestamp, deserialized.timestamp);
        assert_eq!(block.gas_limit, deserialized.gas_limit);
    }

    #[test]
    fn test_indexed_transaction_with_extreme_values() {
        let tx = IndexedTransaction {
            hash: "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string(),
            from: "0x0000000000000000000000000000000000000000".to_string(),
            to: Some("0xffffffffffffffffffffffffffffffffffffffff".to_string()),
            value: "999999999999999999999999999999999999999999999999999999999999999999999".to_string(),
            gas: u64::MAX,
            gas_price: "999999999999999999999999999999999999999999999999999999999999999999999".to_string(),
            input: "0x".to_string().repeat(1000), // Very long input
            nonce: u64::MAX,
            transaction_index: Some(u64::MAX),
        };

        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: IndexedTransaction = serde_json::from_str(&json).unwrap();

        assert_eq!(tx.hash, deserialized.hash);
        assert_eq!(tx.gas, deserialized.gas);
        assert_eq!(tx.nonce, deserialized.nonce);
        assert_eq!(tx.input.len(), deserialized.input.len());
    }

    #[test]
    fn test_indexed_block_complete_round_trip() {
        let block = IndexedBlock {
            number: 12345,
            hash: "0xabc123".to_string(),
            parent_hash: "0xdef456".to_string(),
            timestamp: 1609459200,
            gas_limit: 15000000,
            gas_used: 8000000,
            miner: Some("0xminer123".to_string()),
            difficulty: "1234567890".to_string(),
            total_difficulty: "9876543210".to_string(),
            size: 25000,
            transactions: vec![
                IndexedTransaction {
                    hash: "0xtx1".to_string(),
                    from: "0xfrom1".to_string(),
                    to: Some("0xto1".to_string()),
                    value: "1000000000000000000".to_string(),
                    gas: 21000,
                    gas_price: "20000000000".to_string(),
                    input: "0x123456".to_string(),
                    nonce: 5,
                    transaction_index: Some(0),
                },
            ],
            transaction_count: 1,
            uncles: 2,
            indexed_at: 1609459200,
        };

        // Serialize
        let json = serde_json::to_string(&block).unwrap();
        
        // Deserialize
        let deserialized: IndexedBlock = serde_json::from_str(&json).unwrap();

        // Verify all fields
        assert_eq!(block.number, deserialized.number);
        assert_eq!(block.hash, deserialized.hash);
        assert_eq!(block.parent_hash, deserialized.parent_hash);
        assert_eq!(block.timestamp, deserialized.timestamp);
        assert_eq!(block.gas_limit, deserialized.gas_limit);
        assert_eq!(block.gas_used, deserialized.gas_used);
        assert_eq!(block.miner, deserialized.miner);
        assert_eq!(block.difficulty, deserialized.difficulty);
        assert_eq!(block.total_difficulty, deserialized.total_difficulty);
        assert_eq!(block.size, deserialized.size);
        assert_eq!(block.transaction_count, deserialized.transaction_count);
        assert_eq!(block.uncles, deserialized.uncles);
        assert_eq!(block.indexed_at, deserialized.indexed_at);
        assert_eq!(block.transactions.len(), deserialized.transactions.len());
    }

    #[test]
    fn test_indexed_transaction_with_empty_strings() {
        let tx = IndexedTransaction {
            hash: "".to_string(),
            from: "".to_string(),
            to: Some("".to_string()),
            value: "".to_string(),
            gas: 0,
            gas_price: "".to_string(),
            input: "".to_string(),
            nonce: 0,
            transaction_index: Some(0),
        };

        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: IndexedTransaction = serde_json::from_str(&json).unwrap();

        assert_eq!(tx.hash, deserialized.hash);
        assert_eq!(tx.from, deserialized.from);
        assert_eq!(tx.value, deserialized.value);
    }

    #[test]
    fn test_indexed_block_with_many_transactions() {
        let transactions: Vec<IndexedTransaction> = (0..100)
            .map(|i| IndexedTransaction {
                hash: format!("0x{:064x}", i),
                from: format!("0x{:040x}", i),
                to: Some(format!("0x{:040x}", i + 1000)),
                value: i.to_string(),
                gas: 21000 + i,
                gas_price: (i * 1000).to_string(),
                input: format!("0x{:02x}", i % 256),
                nonce: i,
                transaction_index: Some(i),
            })
            .collect();

        let block = IndexedBlock {
            number: 9999,
            hash: "0xblock".to_string(),
            parent_hash: "0xparent".to_string(),
            timestamp: 1234567890,
            gas_limit: 30000000,
            gas_used: 20000000,
            miner: Some("0xminer".to_string()),
            difficulty: "5000".to_string(),
            total_difficulty: "10000".to_string(),
            size: 100000,
            transactions: transactions.clone(),
            transaction_count: 100,
            uncles: 0,
            indexed_at: 1234567890,
        };

        assert_eq!(block.transactions.len(), 100);
        assert_eq!(block.transaction_count, 100);
        assert_eq!(block.transactions[0].hash, "0x0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(block.transactions[99].hash, "0x0000000000000000000000000000000000000000000000000000000000000063");
    }
}
