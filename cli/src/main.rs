use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod build;
mod config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // /// Optional name to operate on
    // name: Option<String>,

    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile to bytecode
    Build {
        #[arg(short, long, default_value = "./target")]
        out_dir: PathBuf,
        // /// lists test values
        // #[arg(short, long)]
        // path: Option<PathBuf>,
    },

    /// Deploy bytecode
    Deploy {
        path: Option<PathBuf>,
        // /// lists test values
        // #[arg(short, long)]
        // path: Option<PathBuf>,
    },
}

fn main() {
    if let Err(e) = processor() {
        eprint!("IO Error: {}", e.to_string())
    }
}

fn processor() -> std::io::Result<()> {
    let current_dir = config::current_dir()?;
    let cfg = config::read_config(&current_dir)?;

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Build { out_dir }) => {
            build::build(&cfg.project.name, out_dir);
        }
        Some(Commands::Deploy { path }) => {
            dbg!(path);
        }
        None => {}
    }
    Ok(())
}
