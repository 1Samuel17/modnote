# Contributing to Rust CLI Template

Thank you for your interest in contributing to this project! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/rust-cli-template.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes thoroughly
6. Commit your changes: `git commit -m 'Add some feature'`
7. Push to your fork: `git push origin feature/your-feature-name`
8. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- Make (optional)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Style

This project follows the standard Rust style guidelines:

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Write clear, self-documenting code
- Add comments for complex logic

### Making Changes

1. **Keep changes focused**: Each PR should address a single concern
2. **Write tests**: Add tests for new features or bug fixes
3. **Update documentation**: Update README.md and code comments as needed
4. **Follow conventions**: Match the existing code style
5. **Update CHANGELOG.md**: Add your changes under the "Unreleased" section

## Pull Request Process

1. Ensure all tests pass (`cargo test`)
2. Ensure code is formatted (`cargo fmt`)
3. Ensure no clippy warnings (`cargo clippy`)
4. Update the CHANGELOG.md with your changes
5. Update the README.md if you're adding/changing features
6. Your PR will be reviewed by maintainers
7. Address any feedback from reviewers
8. Once approved, your PR will be merged

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors.

### Our Standards

- Be respectful and inclusive
- Be patient and kind
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, trolling, or insulting comments
- Personal or political attacks
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

## Reporting Issues

If you find a bug or have a feature request:

1. Check if it's already reported in Issues
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce (for bugs)
   - Expected vs actual behavior
   - Your environment (OS, Rust version, etc.)

## Questions?

Feel free to open an issue with the "question" label if you need help or clarification.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
