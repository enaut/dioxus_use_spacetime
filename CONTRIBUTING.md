# Contributing to dioxus_use_spacetime

Thank you for your interest in contributing to dioxus_use_spacetime! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/dioxus_use_spacetime.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -m "Description of your changes"`
7. Push to your fork: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
cargo run --example basic_usage
```

## Code Style

We follow the standard Rust style guidelines:

- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Write documentation for public APIs
- Include tests for new functionality

## Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Update documentation as needed
- Add tests for new functionality
- Ensure all tests pass
- Update CHANGELOG.md if applicable
- Follow conventional commit messages

## Commit Message Format

We use conventional commit messages:

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `test:` for test changes
- `refactor:` for code refactoring
- `chore:` for maintenance tasks

## Questions?

Feel free to open an issue if you have questions or need help!

## Code of Conduct

Please be respectful and constructive in all interactions. We aim to maintain a welcoming and inclusive community.
