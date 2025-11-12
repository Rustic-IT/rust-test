# System Integrity Reminder

A cross-platform desktop application built with Tauri and Rust that presents an always-on-top overlay window to remind users to maintain their computer's integrity and security by rebooting their system when it has been running for an extended period.

## Features

- ⚡ **Always-on-top overlay window** - Ensures visibility to users
- 📝 **Customizable messages** - Configure reminder text to suit your needs
- ⏰ **Condition-based triggers** - Set uptime thresholds (e.g., show after 7 days)
- 🔄 **Actionable buttons** - Reboot now, snooze for later, or dismiss
- 🖥️ **Cross-platform** - Supports Windows and macOS
- 🦀 **Built with Rust** - Using the Tauri framework for performance and security

## Prerequisites

### For Development

- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://www.rust-lang.org/) (latest stable)
- [Tauri Prerequisites](https://tauri.app/start/prerequisites/) for your OS

### Platform-Specific Requirements

#### Windows
- Microsoft Visual C++ Build Tools
- WebView2 (usually pre-installed on Windows 10/11)

#### macOS
- Xcode Command Line Tools
- macOS 10.15 or later

## Installation

1. Clone this repository:
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

4. Build for production:
```bash
npm run tauri build
```

## Configuration

The application can be configured using a `config.json` file. See `config.example.json` for the format:

```json
{
  "message": "Your system has been running for an extended period. Please consider rebooting to maintain system integrity and security.",
  "uptime_threshold_days": 7,
  "check_uptime": true
}
```

### Configuration Options

- **message**: The text displayed to users in the reminder window
- **uptime_threshold_days**: Number of days before the reminder appears (default: 7)
- **check_uptime**: Whether to check system uptime before showing (default: true)

## Usage

### For End Users

1. Launch the application
2. If your system uptime exceeds the configured threshold, the reminder window will appear
3. Choose an action:
   - **Reboot Now**: Initiates a system reboot (with confirmation)
   - **Snooze (4 hours)**: Hides the reminder for 4 hours
   - **Dismiss**: Closes the reminder window

### For Administrators

You can deploy this application with custom configuration:

1. Modify `config.json` with your desired settings
2. Build the application using `npm run tauri build`
3. Distribute the built application to your users
4. The application will automatically check system uptime on startup

## Architecture

### Backend (Rust)
- `src-tauri/src/lib.rs`: Main application logic
  - System uptime detection using `sysinfo` crate
  - Configuration management
  - Window management (always-on-top)
  - Cross-platform system commands (reboot)

### Frontend (HTML/CSS/JavaScript)
- `src/index.html`: UI structure
- `src/styles.css`: Modern, gradient-based styling
- `src/main.js`: Frontend logic and Tauri command invocations

## Development

### Project Structure

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
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── config.example.json    # Example configuration
└── package.json           # Node.js dependencies
```

### Adding New Features

1. Backend commands: Add new `#[tauri::command]` functions in `src-tauri/src/lib.rs`
2. Frontend: Update UI in `src/index.html` and logic in `src/main.js`
3. Register commands in the `invoke_handler` in `lib.rs`

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Security

- The application requires elevated privileges to reboot the system
- On Windows: Uses `shutdown` command
- On macOS: Uses AppleScript with user authentication

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

For issues or questions, please open an issue on the GitHub repository.
