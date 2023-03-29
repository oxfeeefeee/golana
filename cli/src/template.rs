use anchor_syn::idl::Idl;
use anyhow::Result;
use dirs;
use heck::{CamelCase, MixedCase};
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

pub fn test_script(name: &str) -> String {
    format!(
        r#" todo: xxxx {}
"#,
        name
    )
}

pub fn main_dot_go() -> String {
    r#"package main
    
import (
	. "solana"
)

// This is the definition of the MyInstruction
type MyInstruction struct {
	// First, list all the accounts that are used by the instruction
	// Use _signer suffix for accounts that are used as signer
    // ...

	// Second, declare the data stored in the accounts, that needs to be read or written by the instruction
	// Use the account name as prefix, and _dataXXXX suffix:
	// - dataInit for the data that is initialized by the instruction
	// - data for the data that is read by the instruction
	// - dataMut for the data that is written by the instruction
	// ...

	// Finally, list all the instruction parameters
    // ...
}

// This is the business logic of the MyInstruction
func (ix *MyInstruction) Process() {	
    // ...
}

// This is the entry point of the program
func main() {
	GetIx().Process()
}

"#.to_owned()
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

// Taken from anchor-syn/src/idl.rs
pub fn idl_ts(idl: &Idl) -> Result<String> {
    let mut idl = idl.clone();
    for acc in idl.accounts.iter_mut() {
        acc.name = acc.name.to_mixed_case();
    }
    let idl_json = serde_json::to_string_pretty(&idl)?;
    Ok(format!(
        r#"export type {} = {};

export const IDL: {} = {};
"#,
        idl.name.to_camel_case(),
        idl_json,
        idl.name.to_camel_case(),
        idl_json
    ))
}
