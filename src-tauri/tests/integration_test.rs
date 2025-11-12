// Integration tests for System Integrity Reminder

#[cfg(test)]
mod tests {
    use system_integrity_reminder_lib::{AppConfig};

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.uptime_threshold_days, 7);
        assert!(config.check_uptime);
        assert!(!config.message.is_empty());
    }

    #[test]
    fn test_custom_config() {
        let config = AppConfig {
            message: "Custom message".to_string(),
            uptime_threshold_days: 14,
            check_uptime: false,
        };
        
        assert_eq!(config.message, "Custom message");
        assert_eq!(config.uptime_threshold_days, 14);
        assert!(!config.check_uptime);
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.message, deserialized.message);
        assert_eq!(config.uptime_threshold_days, deserialized.uptime_threshold_days);
        assert_eq!(config.check_uptime, deserialized.check_uptime);
    }
}
