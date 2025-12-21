# Contributing to Blockchain Indexer

Thank you for your interest in contributing! This document provides guidelines for contributing to this project.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](../../issues)
2. If not, create a new issue with:
   - Clear description of the problem
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)

### Suggesting Features

1. Check if the feature has already been suggested
2. Create a new issue describing:
   - The feature and its use case
   - Why it would be useful
   - Any implementation ideas (optional)

### Submitting Code Changes

1. **Fork the repository**
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**:
   - Follow the existing code style
   - Add tests for new features
   - Update documentation if needed
4. **Run tests and checks**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```
5. **Commit your changes**:
   ```bash
   git commit -m "Add: description of your changes"
   ```
6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Create a Pull Request** with a clear description

## Code Style

- Use `cargo fmt` to format your code
- Run `cargo clippy` to check for common issues
- Follow Rust naming conventions
- Add comments for complex logic
- Keep functions focused and small

## Testing

- Add tests for new features
- Ensure all existing tests pass
- Run `cargo test` before submitting

## Questions?

Feel free to open an issue for any questions or discussions!
