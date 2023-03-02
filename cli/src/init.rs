use crate::config::*;
use crate::template;
use crate::util::new_vm_program;
use anchor_client::solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use anchor_client::Program;
use anyhow::Ok;
use anyhow::Result;
use solana_sdk;
use std::fs;
use std::path::Path;

pub fn init(name: &str) -> Result<()> {
    fs::create_dir(name.clone())?;
    std::env::set_current_dir(&name)?;

    let toml = template::golana_toml(name);
    fs::write("Golana.toml", toml)?;

    fs::create_dir("tests")?;

    fs::create_dir("target")?;

    Ok(())
}
