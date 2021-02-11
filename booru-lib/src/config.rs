use anyhow::Context as _;
use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::Write;
use crate::error::BooruError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub account: Option<AccountConfig>,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct AccountConfig {
    pub username: String,
    pub api_key: String,
}

impl Config {
    pub fn account(self, config: AccountConfig) -> Self {
        Self {
            account: Some(config),
            ..self
        }
    }

    fn config_path() -> Result<PathBuf, BooruError> {
        let config_path = if let Some(dir) = env::var_os("BOORU_CONFIG_DIR") {
            dir.into()
        } else {
            dirs::config_dir().with_context(|| "Failed to load config directory")?
        };
        let config_path = config_path.join("booru-config.toml");
        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().unwrap())?;
        }
        Ok(config_path)
    }

    pub fn save(&self) -> Result<(), BooruError> {
        let path = Self::config_path()?;
        let mut file = File::create(&path)?;
        let conf = toml::to_string_pretty(self).with_context(|| "Failed to parse config.")?;
        write!(file, "{}", conf)?;
        file.flush()?;
        Ok(())
    }

    pub fn load() -> Result<Self, BooruError> {
        let path = Self::config_path()?;
        let conf = fs::read_to_string(path)?;
        let conf: Self = toml::from_str(&conf)?;
        Ok(conf)
    }
}