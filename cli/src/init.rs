use crate::template;
use anyhow::{Ok, Result};

use std::fs;

pub fn init(name: &str) -> Result<()> {
    fs::create_dir(name.clone())?;
    std::env::set_current_dir(&name)?;

    let toml = template::golana_toml(name);
    fs::write("Golana.toml", toml)?;

    fs::write("main.go", template::main_dot_go())?;

    let test_dir = "tests";
    fs::create_dir(test_dir)?;

    std::env::set_current_dir(test_dir)?;
    fs::write(&format!("{}.ts", name), template::test_script(name))?;

    fs::create_dir("target")?;

    Ok(())
}
