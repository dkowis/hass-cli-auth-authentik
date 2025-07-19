use serde::Deserialize;
use std::collections::HashMap;
use std::{env, fs};
use std::path::PathBuf;
use crate::prelude::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ldap_url: String,
    pub bind_dn: String,
    pub bind_pw: String,
    pub user_base_dn: String,
    pub groups: Vec<String>,
    #[serde(default = "default_mappings")]
    pub mappings: HashMap<String, Vec<String>>,
    #[serde(default = "default_username_attribute")]
    pub username_attribute: String,
    #[serde(default = "default_attributes")]
    pub attributes: Vec<String>,
    #[serde(default = "default_timeout")]
    pub timeout: usize,
}

fn default_mappings() -> HashMap<String, Vec<String>> {
    HashMap::new()
}

fn default_username_attribute() -> String {
    "cn".to_string()
}

fn default_attributes() -> Vec<String> {
    vec![]
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
        let abs_path  = if path.is_absolute() {
            path
        } else {
            env::current_dir()
                .context("Unable to get current directory")?
                .join(path)
        };

        //load in the config file!
        let data = fs::read_to_string(&abs_path)
            .with_context(|| format!("Unable to read config file at {:?}", abs_path))?;

        let config = toml::from_str(&data).context("Failed to parse TOML config")?;
        Ok(config)
    }
}
