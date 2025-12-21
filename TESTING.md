# Testing Guide

This document explains how to run tests in the blockchain-indexer project.

## Quick Start

Run all tests:
```bash
cargo test
```

## Test Commands

### Run All Tests
```bash
cargo test
```

### Run Tests with Output
Show output from `println!` and other print statements:
```bash
cargo test -- --nocapture
```

### Run a Specific Test
Run only one test by name:
```bash
cargo test test_blocks_index
```

### Run Tests in a Specific Module
Run all tests in the `config` module:
```bash
cargo test config::
```

### Run Tests with Verbose Output
See more detailed information:
```bash
cargo test --verbose
```

### Run Tests in Parallel (default)
Tests run in parallel by default. To run sequentially:
```bash
cargo test -- --test-threads=1
```

## Test Structure

Tests are organized in modules using `#[cfg(test)]`:

- **`src/config.rs`** - Tests for configuration (8 tests)
  - `test_blocks_index` - Tests block index name generation
  - `test_meta_index` - Tests metadata index name generation
  - `test_index_names_with_different_prefixes` - Tests various prefix scenarios
  - `test_config_with_credentials` - Tests config with authentication
  - `test_index_names_with_special_characters` - Tests special characters in prefixes
  - `test_config_without_credentials` - Tests config without authentication
  - `test_config_with_partial_credentials` - Tests config with partial credentials

- **`src/models.rs`** - Tests for data models (11 tests)
  - `test_indexed_block_serialization` - Tests block JSON serialization
  - `test_indexed_transaction_serialization` - Tests transaction JSON serialization
  - `test_indexed_block_with_transactions` - Tests blocks with multiple transactions
  - `test_indexed_transaction_with_none_to` - Tests contract creation transactions
  - `test_indexed_block_without_miner` - Tests blocks without miner info
  - `test_indexed_transaction_without_index` - Tests transactions without index
  - `test_indexed_block_with_extreme_values` - Tests blocks with maximum u64 values
  - `test_indexed_transaction_with_extreme_values` - Tests transactions with extreme values
  - `test_indexed_block_complete_round_trip` - Tests complete serialization/deserialization
  - `test_indexed_transaction_with_empty_strings` - Tests transactions with empty strings
  - `test_indexed_block_with_many_transactions` - Tests blocks with 100 transactions

- **`src/error.rs`** - Tests for error types (3 tests)
  - `test_elasticsearch_error` - Tests Elasticsearch error formatting
  - `test_rpc_error` - Tests RPC error formatting
  - `test_serialization_error` - Tests serialization error formatting

## Understanding Test Output

### Successful Test Run
```
running 21 tests
test config::tests::test_blocks_index ... ok
test error::tests::test_rpc_error ... ok
...
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Failed Test
```
test config::tests::test_blocks_index ... FAILED

failures:

---- config::tests::test_blocks_index stdout ----
thread 'config::tests::test_blocks_index' panicked at 'assertion failed: ...'
```

## Writing Tests

Tests use the `#[test]` attribute:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Arrange: Set up test data
        let value = 42;
        
        // Act: Execute the code being tested
        let result = function_to_test(value);
        
        // Assert: Verify the result
        assert_eq!(result, expected_value);
    }
}
```

### Common Assertions

- `assert_eq!(a, b)` - Asserts two values are equal
- `assert_ne!(a, b)` - Asserts two values are not equal
- `assert!(condition)` - Asserts a condition is true
- `assert!(value.is_some())` - Asserts an Option is Some
- `assert!(value.is_none())` - Asserts an Option is None

## Continuous Integration

Tests are automatically run in CI/CD via GitHub Actions (`.github/workflows/ci.yml`):
- On every push to `main` or `master`
- On every pull request
- Checks formatting, runs clippy, and runs all tests

## Best Practices

1. **Test naming**: Use descriptive names like `test_blocks_index` not `test1`
2. **One assertion per test**: Focus each test on one behavior
3. **Test edge cases**: Test `None` values, empty vectors, etc.
4. **Keep tests fast**: Unit tests should run quickly
5. **Test behavior, not implementation**: Test what the code does, not how
