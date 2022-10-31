use goscript_engine as gos;
use std::io::{Result, Write};
use std::path::Path;

pub fn build(proj_name: &str, out_dir: &Path) {
    let reader =
        gos::run_fs::FsReader::new(Some(Path::new("./")), Some(Path::new("../../lib/")), None);
    let engine = gos::Engine::new();
    match engine.compile_serialize(false, false, &reader, Path::new("./main.go")) {
        Ok(data) => {
            write_file(proj_name, out_dir, &data).expect("Unable to write file");
        }
        Err(el) => {
            el.sort();
            eprint!("{}", el);
        }
    }
}

fn write_file(proj_name: &str, out_dir: &Path, data: &Vec<u8>) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;

    let file_name = format!("{}.gosb", proj_name);
    let mut full_name = out_dir.to_owned();
    full_name.push(file_name);

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(full_name)?;
    f.write_all(data)?;
    f.flush()
}
