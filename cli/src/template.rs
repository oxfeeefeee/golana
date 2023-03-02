use dirs;
use std::path::PathBuf;

pub fn golana_toml(name: &str) -> String {
    format!(
        r#"[project]
name = "{0}"
space = 30000
out_dir = "./target"

[provider]
cluster = "localnet"
wallet = "{1}"
golana_id = "6ZjLk7jSFVVb2rxeoRf4ex3Q7zECi5SRTV4HbX55nNdP"
    "#,
        name,
        get_wallet_path()
    )
}

fn get_wallet_path() -> String {
    let home_dir = dirs::home_dir().unwrap_or_else(|| {
        println!("$HOME doesn't exist");
        PathBuf::from(".")
    });
    let mut wallet_path = home_dir.to_str().unwrap().to_string();
    wallet_path.push_str("/.config/solana/id.json");
    wallet_path
}
