# Contributing to System Integrity Reminder

Thank you for your interest in contributing to the System Integrity Reminder project! This document provides guidelines for contributing to the project.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:

1. **Clear title and description**
2. **Steps to reproduce** the issue
3. **Expected behavior** vs actual behavior
4. **System information**:
   - Operating system and version
   - Application version
   - Configuration (if relevant)
5. **Screenshots or logs** (if applicable)

### Suggesting Enhancements

We welcome feature suggestions! Please open an issue with:

1. **Clear description** of the feature
2. **Use case** - why is this feature needed?
3. **Proposed implementation** (if you have ideas)
4. **Examples** from other applications (if relevant)

### Pull Requests

We love pull requests! Here's how to submit one:

1. **Fork the repository**
2. **Create a feature branch** from `main`
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**
4. **Test your changes** thoroughly
5. **Commit with clear messages**
   ```bash
   git commit -m "Add: Brief description of changes"
   ```
6. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Open a Pull Request** with:
   - Clear description of changes
   - Reference to related issues
   - Screenshots (for UI changes)
   - Test results

## Development Setup

### Prerequisites

- Node.js (v16 or later)
- Rust (latest stable)
- Platform-specific requirements (see README.md)

### Setting Up

1. Clone the repository:
   ```bash
   git clone https://github.com/Rustic-IT/rust-test.git
   cd rust-test
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

## Coding Standards

### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Add tests for new functionality
- Document public APIs with doc comments

Example:
```rust
/// Gets the system uptime in seconds
///
/// # Returns
///
/// Returns a `Result` containing the uptime in seconds or an error message
#[tauri::command]
fn get_system_uptime() -> Result<u64, String> {
    // Implementation
}
```

### JavaScript Code

- Use modern ES6+ syntax
- Add comments for complex logic
- Follow consistent naming conventions
- Handle errors gracefully

Example:
```javascript
/**
 * Format uptime into human-readable string
 * @param {number} seconds - Uptime in seconds
 * @returns {string} Formatted uptime string
 */
function formatUptime(seconds) {
    // Implementation
}
```

### CSS Code

- Use consistent indentation (2 spaces)
- Group related properties
- Add comments for complex layouts
- Use CSS variables for repeated values

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:
```
feat: Add configuration reload without restart
fix: Correct uptime calculation on Windows
docs: Update installation instructions
```

## Testing

### Running Tests

```bash
# Rust tests
cd src-tauri
cargo test

# Check code with clippy
cargo clippy

# Format code
cargo fmt
```

### Writing Tests

- Add unit tests for new Rust functions
- Test edge cases and error conditions
- Ensure tests are deterministic

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = AppConfig::default();
        assert_eq!(config.uptime_threshold_days, 7);
    }
}
```

## Documentation

- Update README.md for significant changes
- Update USAGE.md for user-facing features
- Add inline code comments for complex logic
- Update this CONTRIBUTING.md if process changes

## Security

- Review SECURITY.md before making changes
- Don't introduce dependencies with known vulnerabilities
- Validate all user inputs
- Never execute arbitrary code from configuration
- Use type-safe deserialization

If you discover a security issue, please report it privately rather than opening a public issue.

## Review Process

1. **Automated checks** run on all PRs:
   - Build verification
   - Code formatting
   - Linting

2. **Manual review** by maintainers:
   - Code quality
   - Security considerations
   - Documentation completeness
   - Test coverage

3. **Feedback** and iteration:
   - Address reviewer comments
   - Make requested changes
   - Push updates to the same branch

4. **Merge**:
   - Approved PRs are merged by maintainers
   - Credit given to all contributors

## Project Structure

```
.
├── src/                    # Frontend files
│   ├── index.html         # Main UI
│   ├── main.js            # Frontend logic
│   └── styles.css         # Styling
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs        # Main application code
│   │   └── main.rs       # Entry point
│   ├── tests/            # Integration tests
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── .github/
│   └── workflows/        # CI/CD workflows
├── config.example.json   # Example configuration
├── README.md             # Project documentation
├── USAGE.md              # User guide
├── SECURITY.md           # Security documentation
└── CONTRIBUTING.md       # This file
```

## Getting Help

- **Questions**: Open a discussion on GitHub
- **Bugs**: Open an issue
- **Security**: See SECURITY.md for reporting process

## Recognition

Contributors will be:
- Listed in release notes
- Credited in commit messages
- Recognized in the project README

Thank you for contributing to System Integrity Reminder! 🎉
