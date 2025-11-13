# Usage Guide - System Integrity Reminder

This guide provides detailed instructions for using and configuring the System Integrity Reminder application.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Running the Application](#running-the-application)
- [User Interface](#user-interface)
- [Administrative Deployment](#administrative-deployment)
- [Troubleshooting](#troubleshooting)

## Installation

### For End Users

1. Download the installer for your operating system from the releases page
2. Run the installer:
   - **Windows**: Double-click the `.msi` or `.exe` installer
   - **macOS**: Open the `.dmg` file and drag the app to Applications

### For Developers

See the main README.md for development setup instructions.

## Configuration

The application uses a `config.json` file to customize its behavior. By default, the application uses built-in settings, but you can override them by creating a configuration file.

### Configuration File Location

Create a `config.json` file in the same directory as the application executable:

- **Windows**: Next to `system-integrity-reminder.exe`
- **macOS**: In the application bundle's Resources folder or in `~/.config/system-integrity-reminder/`

### Configuration Options

```json
{
  "message": "Your system has been running for an extended period. Please consider rebooting to maintain system integrity and security.",
  "uptime_threshold_days": 7,
  "check_uptime": true
}
```

#### Available Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `message` | string | Default reminder message | The text displayed to users in the reminder window |
| `uptime_threshold_days` | number | 7 | Number of days before the reminder appears |
| `check_uptime` | boolean | true | Whether to check system uptime before showing the reminder |

### Example Configurations

#### Weekly Reminder (Default)
```json
{
  "message": "Your system has been running for over a week. Please reboot to ensure security updates are applied.",
  "uptime_threshold_days": 7,
  "check_uptime": true
}
```

#### Monthly Reminder
```json
{
  "message": "Monthly system maintenance is recommended. Please reboot your computer.",
  "uptime_threshold_days": 30,
  "check_uptime": true
}
```

#### Always Show (Testing/Demo)
```json
{
  "message": "System maintenance required.",
  "uptime_threshold_days": 0,
  "check_uptime": false
}
```

## Running the Application

### Windows

1. Launch the application from the Start Menu or Desktop shortcut
2. If your system uptime exceeds the threshold, the reminder window will appear
3. The window will stay on top of other windows to ensure visibility

### macOS

1. Launch the application from Applications or Spotlight
2. If your system uptime exceeds the threshold, the reminder window will appear
3. The window will stay on top of other windows to ensure visibility

### Command Line Options

Currently, the application does not support command-line arguments. All configuration is done through the `config.json` file.

## User Interface

### Reminder Window

The reminder window displays:

- **Warning Icon and Title**: "⚠️ System Maintenance Required"
- **Custom Message**: Your configured reminder message
- **System Uptime**: Current uptime in days and hours
- **Action Buttons**:
  - **Reboot Now**: Initiates an immediate system reboot (with confirmation)
  - **Snooze (4 hours)**: Hides the reminder for 4 hours
  - **Dismiss**: Closes the reminder window

### Button Actions

#### Reboot Now
- Displays a confirmation dialog
- On Windows: Schedules a reboot in 60 seconds using `shutdown /r /t 60`
- On macOS: Uses AppleScript to initiate a reboot (requires admin password)
- Shows a notification with the reboot status

#### Snooze
- Hides the reminder window for 4 hours
- The application continues running in the background
- The reminder will reappear after the snooze period if conditions are still met

#### Dismiss
- Closes the reminder window
- The application continues running in the background
- The reminder will reappear on next application launch if conditions are still met

## Administrative Deployment

For IT administrators deploying this application across multiple machines:

### Group Policy Deployment (Windows)

1. Build the application with your custom configuration
2. Create an MSI installer using the Tauri build process
3. Deploy via Group Policy Software Installation
4. Include a default `config.json` in the deployment package

### MDM Deployment (macOS)

1. Build the application with your custom configuration
2. Create a `.pkg` installer
3. Deploy via your MDM solution (Jamf, Intune, etc.)
4. Use a configuration profile to deploy the `config.json` file

### Startup Configuration

To ensure the application runs at system startup:

#### Windows
- Add a shortcut to the application in the Startup folder
- Or use Task Scheduler to run at logon

#### macOS
- Add the application to Login Items in System Preferences
- Or create a LaunchAgent plist file

### Network Configuration Distribution

For centralized configuration management:

1. Host your `config.json` on a network share
2. Create a startup script that copies the config file to the local machine
3. Launch the application after the config is updated

## Troubleshooting

### The reminder doesn't appear

**Possible causes:**
- System uptime is below the threshold
- `check_uptime` is set to `false` in an incorrect configuration
- Application is not running

**Solutions:**
1. Check your system uptime (Task Manager on Windows, Activity Monitor on macOS)
2. Verify your `config.json` settings
3. Ensure the application is running (check system tray/menu bar)

### Reboot button doesn't work

**Windows:**
- Ensure you have administrative privileges
- Check Windows Event Viewer for error messages

**macOS:**
- You'll be prompted for your admin password
- Ensure you have admin rights on the system

### Window disappears immediately

**Possible causes:**
- System uptime is below threshold
- Configuration error

**Solutions:**
1. Set `check_uptime` to `false` for testing
2. Set `uptime_threshold_days` to `0` for testing
3. Check application logs

### Configuration changes not taking effect

**Solutions:**
1. Ensure `config.json` is in the correct location
2. Verify JSON syntax is correct (use a JSON validator)
3. Restart the application after making changes

### Getting Help

If you encounter issues not covered here:

1. Check the GitHub repository for known issues
2. Open a new issue with:
   - Your operating system and version
   - Application version
   - Configuration file contents
   - Detailed description of the problem
   - Any error messages or logs

## Security Considerations

- The application requires elevated privileges to execute system reboots
- On Windows, the `shutdown` command is used
- On macOS, AppleScript prompts for admin credentials
- Never run the application with configuration files from untrusted sources
- Review the configuration file for any suspicious commands before deployment

## Best Practices

1. **Test First**: Test the application with your configuration in a controlled environment
2. **Gradual Rollout**: Deploy to a small group before organization-wide deployment
3. **User Communication**: Inform users about the application's purpose and behavior
4. **Feedback Loop**: Provide users with a way to report issues or request configuration changes
5. **Regular Review**: Periodically review and adjust the uptime threshold based on your organization's needs
