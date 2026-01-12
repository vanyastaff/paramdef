# Contributing to paramdef

Thank you for your interest in contributing to paramdef! This document provides guidelines and instructions for contributing.

## Getting Started

### Prerequisites

- Rust 1.92 or later (MSRV - Minimum Supported Rust Version)
- `cargo-nextest` for running tests: `cargo install cargo-nextest`
- `cargo-deny` for license checking: `cargo install cargo-deny`

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/yourusername/paramdef.git
cd paramdef

# Check that everything builds
cargo check --workspace --all-targets

# Run tests
cargo nextest run --workspace --all-features

# Run clippy
cargo clippy --workspace --all-features -- -D warnings

# Format code
cargo fmt --all
```

## Development Workflow

### Before You Start

1. Check existing issues and PRs to avoid duplicate work
2. For major changes, open an issue first to discuss the approach
3. Fork the repository and create a new branch

### Making Changes

1. **Write tests first** (TDD approach recommended)
2. **Follow the architecture** described in `docs/01-ARCHITECTURE.md`
3. **Update documentation** for public API changes
4. **Run the full test suite** before submitting

### Code Style

- **Formatting**: Use `rustfmt` (configured in `rustfmt.toml`)
- **Linting**: Fix all `clippy` warnings (configured in `clippy.toml`)
- **Naming**: Follow conventions in `CLAUDE.md`
- **Documentation**: All public APIs must have doc comments

### Testing

```bash
# Run all tests
cargo nextest run --workspace --all-features

# Test specific features
cargo nextest run --workspace --features validation
cargo nextest run --workspace --features visibility

# Run doctests (nextest doesn't run these)
cargo test --workspace --doc

# CI profile (with retries)
cargo nextest run --profile ci
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks
- `perf:` - Performance improvements

**Example:**
```
feat(validation): add email validator with regex caching

- Implement EmailValidator with thread-local regex cache
- Add tests for valid/invalid email formats
- Update validation documentation

Closes #123
```

### Pull Request Process

1. **Update documentation** if you've changed APIs
2. **Add tests** for new functionality
3. **Run the full test suite** and ensure it passes
4. **Update CHANGELOG.md** with your changes
5. **Ensure CI passes** before requesting review

### CI Checks

All PRs must pass:
- ✅ `cargo check --workspace --all-targets`
- ✅ `cargo nextest run --workspace --all-features`
- ✅ `cargo clippy --workspace --all-features -- -D warnings`
- ✅ `cargo fmt --all -- --check`
- ✅ `cargo doc --no-deps --all-features`
- ✅ MSRV check: `cargo +1.92 check --workspace`

## Project Structure

```
paramdef/
├── src/
│   ├── core/           # Key, Value, Metadata, Flags
│   ├── types/          # 23 node types
│   ├── subtype/        # Subtypes and units
│   ├── schema/         # Schema builder
│   ├── context/        # Runtime context
│   ├── event/          # Event system (optional)
│   ├── validation/     # Validation (optional)
│   └── visibility/     # Visibility (optional)
├── docs/               # Architecture and design docs
└── tests/              # Integration tests
```

## Feature Flags

When adding features:
- **Core features**: No dependencies, always available
- **Optional features**: Gate with `#[cfg(feature = "name")]`
- **Update**: `Cargo.toml` features section
- **Test**: All feature combinations in CI

## Architecture Guidelines

1. **Immutable schema** - Never mutate `Arc<T>` types
2. **Separation of concerns** - Schema / Runtime / Value layers
3. **Zero UI dependencies** - Must work headless
4. **Composition over proliferation** - Use flags and subtypes
5. **Type-safe builders** - Compile-time checks where possible

See `docs/17-DESIGN-DECISIONS.md` for rationale.

## Need Help?

- 📖 Read the [documentation](docs/)
- 💬 Open a [discussion](https://github.com/yourusername/paramdef/discussions)
- 🐛 Report bugs via [issues](https://github.com/yourusername/paramdef/issues)

## License

By contributing, you agree that your contributions will be licensed under:
- MIT License
- Apache License 2.0

Thank you for contributing to paramdef! 🎉
