# Rust CLI Template

A comprehensive template for building Rust command-line applications with best practices and modern tooling.

## Features

- 🦀 **Modern Rust**: Built with the latest stable Rust
- 📦 **CLI Framework**: Uses `clap` for argument parsing with derive macros
- 🛠️ **Development Tools**: Makefile for common tasks
- 🔄 **CI/CD**: GitHub Actions workflows for testing and releases
- 🐳 **Dev Container**: VS Code dev container with Rust toolchain
- 📝 **Documentation**: Comprehensive README and CHANGELOG
- 🔒 **Security**: Automated security audits

## Quick Start

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Make (optional, for using Makefile commands)

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/1Samuel17/rust-cli-template.git
   cd rust-cli-template
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the application:
   ```bash
   cargo run -- --name "World"
   ```

### Using as a Template

1. Click "Use this template" on GitHub
2. Clone your new repository
3. Update `Cargo.toml` with your project details
4. Update this README with your project information
5. Start building your CLI application!

## Usage

```bash
# Show help
cargo run -- --help

# Run with custom name
cargo run -- --name "Rust"

# Run with verbose output
cargo run -- --name "Developer" --verbose
```

## Development

### Using Make

This project includes a Makefile for common development tasks:

```bash
make help       # Show available commands
make build      # Build in release mode
make test       # Run tests
make format     # Format code
make lint       # Run clippy
make check      # Run cargo check
make run        # Run the application
make dev        # Build and run in dev mode
make audit      # Run security audit
make all        # Run format, lint, test, and build
```

### Manual Commands

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy

# Build release
cargo build --release

# Install locally
cargo install --path .
```

## Project Structure

```
.
├── .devcontainer/          # VS Code dev container configuration
│   └── devcontainer.json
├── .github/
│   └── workflows/          # GitHub Actions workflows
│       ├── ci.yml          # Continuous Integration
│       └── release.yml     # Release automation
├── src/
│   └── main.rs             # Application entry point
├── Cargo.toml              # Project dependencies and metadata
├── Makefile                # Common development tasks
├── CHANGELOG.md            # Project changelog
├── LICENSE                 # MIT License
└── README.md               # This file
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## CI/CD

This template includes GitHub Actions workflows for:

- **CI Pipeline**: Runs on every push and PR
  - Tests on multiple OS (Ubuntu, Windows, macOS)
  - Tests with stable and beta Rust versions
  - Code formatting checks
  - Clippy linting
  - Security audits

- **Release Pipeline**: Triggered on version tags
  - Builds binaries for multiple platforms
  - Creates GitHub releases
  - Uploads compiled binaries as release assets

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI parsing with [clap](https://github.com/clap-rs/clap)
- Error handling with [anyhow](https://github.com/dtolnay/anyhow)

