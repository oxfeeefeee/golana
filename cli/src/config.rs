use serde_derive::Deserialize;
use std::io;
use std::path::{Path, PathBuf};

pub use std::env::current_dir;

#[derive(Debug, Deserialize)]
pub struct GolanaConfig {
    pub project: Project,
    pub provider: Provider,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub space: u64,
    pub out_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Provider {
    pub cluster: String,
    pub wallet: String,
    pub golana_id: String,
}

pub fn get_full_path(dir: &Path) -> Option<PathBuf> {
    let mut buf = dir.to_owned();
    buf.push("Golana.toml");
    if buf.exists() {
        Some(buf)
    } else {
        None
    }
}

pub fn read_config(dir: &Path) -> io::Result<GolanaConfig> {
    let content = std::fs::read_to_string(&dir)?;
    toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}
