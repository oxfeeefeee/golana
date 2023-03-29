use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod airdrop;
mod build;
mod config;
mod deploy;
mod idl;
mod init;
mod template;
mod test;
mod util;

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
        #[arg(short, long)]
        out_name: Option<String>,
    },

    /// Deploy bytecode
    Deploy {
        #[arg(short, long)]
        path: Option<PathBuf>,
        #[arg(short, long)]
        force: bool,
    },

    /// Airdrop to wallet
    Airdrop {
        #[arg(short, long, default_value = "1000000")]
        amount: Option<u64>,
    },

    /// Initialize a new project
    Init {
        #[arg(short, long)]
        name: String,
    },
    // /// Run the test script
    Test {},
}

fn main() {
    if let Err(e) = processor() {
        eprintln!("Error: {}", e.to_string());
        eprintln!("Details: {:#?}", e);
    }
}

fn processor() -> Result<()> {
    let cli = Cli::parse();
    if cli.command.is_none() {
        print!("Use -h for help\n");
        return Ok(());
    }

    let current_dir = config::current_dir()?;
    if let Some(path) = config::get_full_path(&current_dir) {
        let cfg = config::read_config(&path)?;
        match &cli.command.unwrap() {
            Commands::Build { out_name } => build::build(
                out_name.as_ref().map(|x| &**x),
                &cfg.project.out_dir,
                &cfg.project.name,
            ),
            Commands::Airdrop { amount } => {
                println!(
                    "Airdrop {} lamports to wallet at {}",
                    amount.unwrap(),
                    cfg.get_provider()?.wallet
                );
                airdrop::airdrop(amount.unwrap(), cfg.get_provider()?)
            }
            Commands::Deploy { path, force } => {
                let path = path.clone().unwrap_or_else(|| {
                    // Get default path by adding project name to out_dir
                    let mut path = cfg.project.out_dir.clone();
                    path.push(&cfg.project.name);
                    path.set_extension("gosb");
                    path
                });
                print!("Deploying from path: {}\n", path.to_string_lossy());
                deploy::deploy(&cfg, &path, *force)?;
                print!("Deployed!\n");
                Ok(())
            }
            Commands::Test {} => {
                println!("Testing");
                test::test(&cfg)?;
                Ok(())
            }
            Commands::Init { .. } => {
                println!("Golana project already initialized");
                Ok(())
            }
        }
    } else {
        match &cli.command.unwrap() {
            Commands::Init { name } => init::init(name),
            _ => {
                println!("No Golana.toml found in current directory");
                Ok(())
            }
        }
    }
}
