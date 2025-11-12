const { invoke } = window.__TAURI__.core;

// Format uptime into human-readable string
function formatUptime(seconds) {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  
  if (days > 0) {
    return `${days} day${days !== 1 ? 's' : ''}, ${hours} hour${hours !== 1 ? 's' : ''}`;
  } else if (hours > 0) {
    return `${hours} hour${hours !== 1 ? 's' : ''}, ${minutes} minute${minutes !== 1 ? 's' : ''}`;
  } else {
    return `${minutes} minute${minutes !== 1 ? 's' : ''}`;
  }
}

// Load and display system information
async function loadSystemInfo() {
  try {
    // Get config
    const config = await invoke("get_config");
    document.getElementById("message").textContent = config.message;
    
    // Get uptime
    const uptimeSeconds = await invoke("get_system_uptime");
    document.getElementById("uptime").textContent = formatUptime(uptimeSeconds);
    
    // Check if we should show the reminder
    const shouldShow = await invoke("should_show_reminder", { config });
    console.log("Should show reminder:", shouldShow);
    
  } catch (error) {
    console.error("Error loading system info:", error);
    document.getElementById("message").textContent = "Error loading system information.";
    document.getElementById("uptime").textContent = "N/A";
  }
}

// Handle reboot button click
async function handleReboot() {
  if (confirm("Are you sure you want to reboot your system? This will close all applications.")) {
    try {
      const result = await invoke("reboot_system");
      alert(result);
    } catch (error) {
      alert("Failed to reboot: " + error);
    }
  }
}

// Handle snooze button click
async function handleSnooze() {
  try {
    await invoke("snooze_reminder", { 
      hours: 4 
    });
  } catch (error) {
    console.error("Error snoozing reminder:", error);
    alert("Failed to snooze: " + error);
  }
}

// Handle dismiss button click
async function handleDismiss() {
  try {
    await invoke("dismiss_reminder");
  } catch (error) {
    console.error("Error dismissing reminder:", error);
    alert("Failed to dismiss: " + error);
  }
}

// Initialize the application
window.addEventListener("DOMContentLoaded", () => {
  // Load system info on startup
  loadSystemInfo();
  
  // Attach event listeners to buttons
  document.getElementById("reboot-btn").addEventListener("click", handleReboot);
  document.getElementById("snooze-btn").addEventListener("click", handleSnooze);
  document.getElementById("dismiss-btn").addEventListener("click", handleDismiss);
  
  // Refresh uptime every minute
  setInterval(loadSystemInfo, 60000);
});
