use crate::config::*;
use anyhow::{Context, Result};

pub fn test(config: &GolanaConfig) -> Result<()> {
    let provider = config.get_test_provider()?;
    let cmd = &config.test.script;

    let mut args: Vec<&str> = cmd.split(' ').collect();
    let program = args.remove(0);

    let test_result: Result<_> = {
        std::process::Command::new(program)
            .args(args)
            .env("ANCHOR_PROVIDER_URL", provider.cluster.to_string())
            .env("ANCHOR_WALLET", &*shellexpand::tilde(&provider.wallet))
            .env("GOLANA_LOADER_ID", provider.loader_id.to_string())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .map_err(anyhow::Error::from)
            .context(cmd.clone())
    };

    match test_result {
        Ok(exit) => {
            if !exit.status.success() {
                std::process::exit(exit.status.code().unwrap());
            }
        }
        Err(err) => {
            println!("Failed to run test: {:#}", err);
            return Err(err);
        }
    }

    Ok(())
}
