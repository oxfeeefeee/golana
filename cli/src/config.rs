use serde_derive::Deserialize;
use std::io;
use std::path::Path;

pub use std::env::current_dir;

#[derive(Debug, Deserialize)]
pub struct GolanaConfig {
    pub project: Project,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
}

pub fn read_config(dir: &Path) -> io::Result<GolanaConfig> {
    let mut buf = dir.to_owned();
    buf.push("Golana.toml");

    let content = std::fs::read_to_string(&buf)?;
    toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}
