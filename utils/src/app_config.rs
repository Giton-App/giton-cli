use config::{Config, Environment};
use lazy_static::{__Deref, lazy_static};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::RwLock;

use super::error::Result;
use crate::types::LogLevel;

// CONFIG static variable. It's actually an AppConfig
// inside an RwLock.
lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::builder().build().unwrap());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub log_level: LogLevel,
    pub openai_key: String,
}

impl AppConfig {
    /// Initialize AppConfig.
    pub fn init(default_config: Option<&str>) -> Result<()> {
        let mut settings = Config::builder().build()?;

        // Embed file into executable
        // This macro will embed the configuration file into the
        // executable. Check include_str! for more info.
        if let Some(config_contents) = default_config {
            //let contents = include_str!(config_file_path);
            settings.merge(config::File::from_str(
                config_contents,
                config::FileFormat::Toml,
            ))?;
        }

        // Merge settings with env variables
        settings.merge(Environment::with_prefix("GITON"))?; // TODO: Merge settings with Clap Settings Arguments

        // Save Config to RwLoc
        {
            let mut w = CONFIG.write()?;
            *w = settings;
        }

        Ok(())
    }

    pub fn merge_args(args: clap::ArgMatches) -> Result<()> {
        if args.contains_id("debug") {
            let value: &str = *args.get_one("debug").unwrap();
            AppConfig::set("debug", &value.to_string())?;
        }

        if args.contains_id("log_level") {
            let value: &str = *args.get_one("log_level").unwrap();
            AppConfig::set("log_level", &value.to_string())?;
        }

        Ok(())
    }

    pub fn merge_config(config_file: Option<&Path>) -> Result<()> {
        // Merge settings with config file if there is one
        if let Some(config_file_path) = config_file {
            {
                CONFIG
                    .write()?
                    .merge(config::File::with_name(config_file_path.to_str().unwrap()))?;
            }
        }
        Ok(())
    }

    // Set CONFIG
    pub fn set(key: &str, value: &str) -> Result<()> {
        {
            // Set Property
            CONFIG.write()?.set(key, value)?;
        }

        Ok(())
    }

    // Get a single value
    pub fn get<'de, T>(key: &'de str) -> Result<T>
    where
        T: serde::Deserialize<'de>,
    {
        Ok(CONFIG.read()?.get::<T>(key)?)
    }

    // Get CONFIG
    // This clones Config (from RwLock<Config>) into a new AppConfig object.
    // This means you have to fetch this again if you changed the configuration.
    pub fn fetch() -> Result<AppConfig> {
        // Get a Read Lock from RwLock
        let r = CONFIG.read()?;

        // Clone the Config object
        let config_clone = r.deref().clone();

        // Coerce Config into AppConfig
        let app_config: AppConfig = config_clone.into();
        Ok(app_config)
    }
}

// Coerce Config into AppConfig
impl From<Config> for AppConfig {
    fn from(config: Config) -> Self {
        AppConfig {
            debug: config.get_bool("debug").unwrap(),
            log_level: config.get::<LogLevel>("log_level").unwrap(),
            openai_key: config.get::<String>("openai_key").unwrap().to_string(),
        }
    }
}
