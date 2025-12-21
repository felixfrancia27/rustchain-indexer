# Blockchain Indexer

[![CI](https://github.com/felixfrancia27/rustchain-indexer/workflows/CI/badge.svg)](https://github.com/felixfrancia27/rustchain-indexer/actions)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/felixfrancia27/rustchain-indexer/releases)

Rust service that indexes all blockchain blocks into Elasticsearch.

## Features

- Indexes all blocks from genesis
- Historical sync (backfill)
- Live sync (real-time)
- Stores complete blocks with transactions
- Checkpointing for resuming

## Setup

1. Copy `env.example` to `.env`:

```bash
# Windows PowerShell
Copy-Item env.example .env

# Linux/Mac
cp env.example .env
```

2. Configure environment variables:

- `RPC_HTTP_URL` - Ethereum RPC node URL
- `ES_URL` - Elasticsearch URL
- `ES_USERNAME` - Elasticsearch username (optional)
- `ES_PASSWORD` - Elasticsearch password (optional)
- `INDEX_PREFIX` - Index prefix (default: "workqueue")
- `BATCH_SIZE` - Batch size for indexing (default: 1000)
- `START_BLOCK` - Starting block number (default: 0)
- `SYNC_INTERVAL_SECS` - Sync interval in seconds (default: 2)

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run --release
```

## Development

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Elasticsearch instance
- Ethereum RPC node

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_blocks_index
```

See [TESTING.md](TESTING.md) for detailed testing documentation.

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy

# Run all checks
cargo test && cargo clippy && cargo fmt -- --check
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the [LICENSE](LICENSE) file for details.
