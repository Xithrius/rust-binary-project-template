use color_eyre::eyre::{bail, Error, Result};
use serde::Deserialize;

use crate::utils::pathing::config_path;

#[derive(Deserialize)]
pub struct CompleteConfig {}

impl CompleteConfig {
    pub fn new() -> Result<Self, Error> {
        if let Ok(config_contents) = std::fs::read_to_string(config_path("config.toml")) {
            let config: CompleteConfig = toml::from_str(config_contents.as_str()).unwrap();

            Ok(config)
        } else {
            bail!(
                "Configuration not found. Create a config file at '{}', and see '{}' for an example configuration.",
                config_path("config.toml"),
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}
