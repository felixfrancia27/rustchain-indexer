# Release Notes

## Version 0.1.0 - Initial Release

**Release Date:** 2025-12-21

### Overview

This is the initial release of the Blockchain Indexer, a high-performance Rust service for indexing Ethereum blockchain data into Elasticsearch.

### Key Features

- **Historical Sync**: Index all blocks from genesis block with configurable batch processing
- **Real-time Sync**: Continuously monitor and index new blocks as they are mined
- **Elasticsearch Integration**: Automatic index creation and bulk indexing support
- **Checkpointing**: Resume sync from last indexed block after interruptions
- **Concurrent Processing**: Configurable concurrency for optimal performance
- **Comprehensive Testing**: 21 unit tests covering core functionality

### Technical Stack

- **Language**: Rust 2021 Edition
- **Async Runtime**: Tokio
- **Blockchain**: Ethereum (via ethers-rs)
- **Search Engine**: Elasticsearch 8.x
- **License**: AGPL-3.0

### Getting Started

1. Clone the repository
2. Copy `env.example` to `.env` and configure
3. Run `cargo build --release`
4. Execute `cargo run --release`

### Documentation

- [README.md](README.md) - Setup and usage guide
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [TESTING.md](TESTING.md) - Testing documentation
- [CHANGELOG.md](CHANGELOG.md) - Detailed changelog

### Requirements

- Rust 1.70+
- Elasticsearch instance
- Ethereum RPC node

### Breaking Changes

None (initial release)

### Known Issues

None at this time

### Contributors

Initial release by the project maintainers.
