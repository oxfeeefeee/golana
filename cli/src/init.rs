use crate::template;
use anyhow::{Ok, Result};

use std::fs;

pub fn init(name: &str) -> Result<()> {
    fs::create_dir(name.clone())?;
    std::env::set_current_dir(&name)?;

    let toml = template::golana_toml(name);
    fs::write("Golana.toml", toml)?;

    fs::write("main.go", template::main_dot_go())?;

    fs::write(".gitignore", template::gitignore())?;

    fs::write(".eslintrc", template::eslintrc())?;

    fs::write(".mocharc", template::mocharc())?;

    fs::write("package.json", template::npm_package())?;

    fs::write("tsconfig.json", template::tsconfig())?;

    let test_dir = "tests";
    fs::create_dir(test_dir)?;

    std::env::set_current_dir(test_dir)?;
    fs::write(&format!("{}.ts", name), template::test_script(name))?;

    std::env::set_current_dir("../")?;
    fs::create_dir("target")?;

    Ok(())
}
