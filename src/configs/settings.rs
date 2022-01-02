use crate::constants::*;

enum ConfigType {
    Development,
    Staging,
    Production,
}

struct AppSettings {
    config: ConfigType,
    interface: String,
    port: u16,
    log_level: String,
    log_level_web: String,
    log_level_server: String,
    pandoc_binary: String,
    pandoc_workdir: String,
}

impl AppSettings {
    fn new() -> AppSettings {
        AppSettings {
            config: ConfigType::Production,
            interface: DEFAULT_INTERFACE.to_string(),
            port: DEFAULT_PORT,
            log_level: "error".to_string(),
            log_level_web: "info".to_string(),
            log_level_server: "error".to_string(),
            pandoc_binary: DEFAULT_PANDOC_BINARY.to_string(),
            pandoc_workdir: DEFAULT_WORKDIR.to_string()
        }
    }
}