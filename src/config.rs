use crate::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub authentik_base_url: String,
    pub flow_slug: String,
    #[serde(default = "default_timeout")]
    pub timeout: usize,
    pub admin_group_name: Option<String>,
    pub user_group_name: Option<String>,
}

fn default_timeout() -> usize {
    10
}

fn expand_tilde(path: &str) -> Result<String> {
    let binding = dirs::home_dir().context("Failed to get home directory")?;
    let replacement = binding
        .to_str()
        .context("Failed to convert home directory to string")?;
    Ok(path.replace("~", replacement))
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let expanded_path = expand_tilde(path)?;
        let path = PathBuf::from(expanded_path);
        let abs_path = if path.is_absolute() {
            path
        } else {
            env::current_dir()
                .context("Unable to get current directory")?
                .join(path)
        };

        //load in the config file!
        let data = fs::read_to_string(&abs_path)
            .with_context(|| format!("Unable to read config file at {abs_path:?}"))?;

        let config = toml::from_str(&data).context("Failed to parse TOML config")?;
        Ok(config)
    }
}
