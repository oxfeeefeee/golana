use anyhow::{anyhow, Result};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

pub use std::env::current_dir;

#[derive(Debug, Deserialize)]
pub struct GolanaConfig {
    pub project: Project,
    pub providers: HashMap<String, Provider>,
    pub test: Test,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub space: u64,
    pub cache_space: u64,
    pub out_dir: PathBuf,
    pub provider: String,
}

#[derive(Debug, Deserialize)]
pub struct Provider {
    pub cluster: String,
    pub wallet: String,
    pub loader_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub script: String,
}

impl GolanaConfig {
    pub fn get_provider(&self) -> Result<&Provider> {
        self.get_provider_impl(&self.project.provider)
    }

    fn get_provider_impl(&self, key: &str) -> Result<&Provider> {
        self.providers
            .get(key)
            .ok_or_else(|| (anyhow!("Couldn't find provider config with key {:?}", key)))
    }
}

pub fn get_full_path(dir: &Path) -> Option<PathBuf> {
    let buf = dir.join("Golana.toml");
    if buf.exists() {
        Some(buf)
    } else {
        None
    }
}

pub fn read_config(dir: &Path) -> io::Result<GolanaConfig> {
    let content = std::fs::read_to_string(&dir)?;
    Ok(toml::from_str(&content)?)
}
