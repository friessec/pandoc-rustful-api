use crate::constants::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub enum ConfigType {
    Development,
    Staging,
    Production,
}

impl Default for ConfigType {
    fn default() -> Self
    {
        ConfigType::Production
    }
}


#[derive(Clone, Deserialize, Debug)]
pub struct PandocSettings {
    pub binary: String,
    pub workdir: String,
}

impl Default for PandocSettings {
    fn default() -> Self
    {
        Self {
            binary: DEFAULT_PANDOC_BINARY.to_string(),
            workdir: DEFAULT_WORKDIR.to_string(),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct AppSettings {
    #[serde(default)]
    pub config: ConfigType,
    #[serde(default)]
    pub interface: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub log_level: String,
    #[serde(default)]
    pub log_level_web: String,
    #[serde(default)]
    pub log_level_server: String,
    #[serde(default)]
    pub pandoc: PandocSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            config: ConfigType::Production,
            interface: DEFAULT_INTERFACE.to_string(),
            port: DEFAULT_PORT,
            log_level: "error".to_string(),
            log_level_web: "info".to_string(),
            log_level_server: "error".to_string(),
            pandoc: PandocSettings {
                binary: DEFAULT_PANDOC_BINARY.to_string(),
                workdir: DEFAULT_WORKDIR.to_string(),
            }
        }
    }
}

impl AppSettings {
    pub fn new() -> Self {
        AppSettings::default()
    }

    pub fn load(&mut self, data: &str) -> Result<Self, ()> {
        let settings: AppSettings = toml::from_str(data).unwrap();
        let settings = self.merge(settings);
        Ok(settings)
    }

    fn merge(&self, mut _settings: AppSettings) -> AppSettings {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::configs::settings::AppSettings;
    use crate::{DEFAULT_INTERFACE, DEFAULT_PORT};

    #[test]
    fn load_default_settings() {
        let settings = AppSettings::new();

        assert_eq!(settings.port, DEFAULT_PORT);
        assert_eq!(settings.log_level, "error");
        assert_eq!(settings.log_level_web, "info");
        assert_eq!(settings.interface, DEFAULT_INTERFACE);
        assert_eq!(settings.pandoc.binary, "/usr/bin/pandoc");
        assert_eq!(settings.pandoc.workdir, "/tmp/pandoc-workdir");
    }

    // #[test]
    // fn load_merged_settings() {
    //     let mut settings = AppSettings::new();
    //     settings = settings.load(r#"
    //     port = 4000
    //     interface = "127.0.1.1"
    //     log_level = "debug"
    //     "#).unwrap();
    //
    //     assert_eq!(settings.port, 4000);
    //     assert_eq!(settings.log_level, "debug");
    //     assert_eq!(settings.log_level_web, "info");
    //     assert_eq!(settings.interface, "127.0.1.1");
    //     assert_eq!(settings.pandoc.binary, "/usr/bin/pandoc");
    //     assert_eq!(settings.pandoc.workdir, "/tmp/pandoc-workdir");
    // }
}