# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-21

### Added
- Initial release
- Historical block synchronization from genesis block
- Real-time block synchronization
- Elasticsearch integration with automatic index creation
- Checkpointing system for resuming sync after interruptions
- Configurable batch processing for optimal performance
- Concurrent block processing with configurable concurrency limits
- Bulk indexing support for improved Elasticsearch performance
- Comprehensive test suite (21 unit tests)
- Full documentation (README, CONTRIBUTING, TESTING)
- CI/CD pipeline with GitHub Actions
- Support for authenticated Elasticsearch connections
- Configurable index prefixes
- Complete block and transaction data indexing

### Technical Details
- Built with Rust and Tokio for async operations
- Uses ethers-rs for Ethereum RPC communication
- Elasticsearch 8.x compatible
- AGPL-3.0 licensed

## [Unreleased]

### Planned
- Additional blockchain network support
- Performance optimizations
- Enhanced error handling and retry logic
