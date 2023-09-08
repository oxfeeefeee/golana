use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use zip::write::FileOptions;

const GO_LIB_DIR: &str = "./go";
const GO_LIB_ZIP: &str = "go_lib.zip";

fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!("Running cli/build.rs from {}", cwd.display());
    println!("cargo:rerun-if-changed={}", GO_LIB_DIR);

    let dest_path = Path::new(&cwd).join(GO_LIB_ZIP);
    println!(
        "Packing files from {} to {}",
        GO_LIB_DIR,
        dest_path.display()
    );

    zip_go_lib(Path::new(GO_LIB_DIR), &dest_path).unwrap();
}

fn zip_go_lib<P: AsRef<Path>>(src: P, dest: P) -> Result<()> {
    // Create the destination file and ZipWriter.
    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&dest)?;
    let walkdir = WalkDir::new(src.as_ref());
    let it = walkdir.into_iter();
    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        src,
        file,
        zip::CompressionMethod::Stored,
    )?;

    Ok(())
}

fn zip_dir<T, P: AsRef<Path>>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: P,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix.as_ref()).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
