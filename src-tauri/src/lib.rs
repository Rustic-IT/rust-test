use serde::{Deserialize, Serialize};
use sysinfo::System;
use tauri::{AppHandle, Manager, Window};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub message: String,
    pub uptime_threshold_days: u64,
    pub check_uptime: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            message: "Your system has been running for an extended period. Please consider rebooting to maintain system integrity and security.".to_string(),
            uptime_threshold_days: 7,
            check_uptime: true,
        }
    }
}

// Get system uptime in seconds
#[tauri::command]
fn get_system_uptime() -> Result<u64, String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    Ok(System::uptime())
}

// Get system uptime in days
#[tauri::command]
fn get_uptime_days() -> Result<u64, String> {
    let uptime_seconds = System::uptime();
    Ok(uptime_seconds / 86400) // Convert seconds to days
}

// Check if uptime exceeds threshold
#[tauri::command]
fn should_show_reminder(config: AppConfig) -> Result<bool, String> {
    if !config.check_uptime {
        return Ok(true); // Always show if check is disabled
    }
    
    let uptime_days = System::uptime() / 86400;
    Ok(uptime_days >= config.uptime_threshold_days)
}

// Get the current config
#[tauri::command]
fn get_config() -> Result<AppConfig, String> {
    // In a production app, this would load from a config file
    // For now, return default config
    Ok(AppConfig::default())
}

// Update config (in memory for now)
#[tauri::command]
fn update_config(config: AppConfig) -> Result<(), String> {
    // In a production app, this would save to a config file
    // For now, just return success
    println!("Config updated: {:?}", config);
    Ok(())
}

// Reboot system command
#[tauri::command]
async fn reboot_system() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        match Command::new("shutdown").args(["/r", "/t", "60", "/c", "System reboot requested by System Integrity Reminder"]).spawn() {
            Ok(_) => Ok("System will reboot in 60 seconds".to_string()),
            Err(e) => Err(format!("Failed to initiate reboot: {}", e)),
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        // On macOS, osascript with AppleScript is used for better user experience
        // This will prompt for admin password
        match Command::new("osascript")
            .args(["-e", "tell application \"System Events\" to restart"])
            .spawn() {
            Ok(_) => Ok("System reboot initiated".to_string()),
            Err(e) => Err(format!("Failed to initiate reboot: {}", e)),
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        Err("Reboot not supported on this platform".to_string())
    }
}

// Close/hide the reminder window
#[tauri::command]
fn dismiss_reminder(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

// Snooze reminder (hide for a period)
#[tauri::command]
fn snooze_reminder(window: Window, hours: u64) -> Result<(), String> {
    println!("Reminder snoozed for {} hours", hours);
    window.hide().map_err(|e| e.to_string())
}

// Show the reminder window
#[tauri::command]
fn show_reminder(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_system_uptime,
            get_uptime_days,
            should_show_reminder,
            get_config,
            update_config,
            reboot_system,
            dismiss_reminder,
            snooze_reminder,
            show_reminder,
        ])
        .setup(|app| {
            // Get the main window
            if let Some(window) = app.get_webview_window("main") {
                // Set window to be always on top
                window.set_always_on_top(true)?;
                
                // Check if we should show the reminder on startup
                let uptime_days = System::uptime() / 86400;
                let config = AppConfig::default();
                
                if config.check_uptime && uptime_days >= config.uptime_threshold_days {
                    window.show()?;
                    window.set_focus()?;
                } else {
                    // Hide window if conditions not met
                    window.hide()?;
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
