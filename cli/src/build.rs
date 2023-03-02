use anyhow::{anyhow, Context, Result};
use borsh::ser::BorshSerialize;
use golana;
use goscript_engine as gos;
use std::io::Write;
use std::path::Path;

pub fn build(out_name: &str, out_dir: &Path) -> Result<()> {
    let reader =
        gos::run_fs::FsReader::new(Some(Path::new("./")), Some(Path::new("../../lib/")), None);
    let engine = gos::Engine::new();
    let (bc, _) = engine
        .compile(false, false, &reader, Path::new("./main.go"))
        .map_err(|el| {
            el.sort();
            anyhow!(el.to_string())
        })
        .context("compile error")?;

    golana::check(&bc).map_err(|e| anyhow::Error::new(e).context("type check error"))?;

    let buf = bc.try_to_vec().unwrap();
    write_file(out_name, out_dir, &buf)
        .map_err(|e| anyhow::Error::new(e).context("write file error"))
}

fn write_file(out_name: &str, out_dir: &Path, data: &Vec<u8>) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir)?;

    let mut full_name = out_dir.to_owned();
    full_name.push(out_name);

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(full_name)?;
    f.write_all(data)?;
    f.flush()
}
