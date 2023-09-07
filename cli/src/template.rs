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
cache_space = 300000
out_dir = "./target"
provider = "testnet"

[providers.localnet]
cluster = "http://127.0.0.1:8899"
wallet = "{1}"
loader_id = "HE7R2wfjpgjHnxfA9bS6fSLJzm7nucFfBXQhhxTCWMZs"

[providers.testnet]
cluster = "https://api.testnet.solana.com"
wallet = "{1}"
loader_id = "HE7R2wfjpgjHnxfA9bS6fSLJzm7nucFfBXQhhxTCWMZs"

[test]
script = "npx mocha -t 1000000 tests/**/*.ts"
"#,
        name,
        get_wallet_path()
    )
}

pub fn test_script(name: &str) -> String {
    format!(
        r#"import {{ IDL, {0} }} from '../target/{1}_idl.js';
import {{ Program, initFromEnv }} from "golana";
import {{ Keypair, SystemProgram }} from '@solana/web3.js';
import BN from 'bn.js';

describe("{1}", async () => {{
    try {{
        const provider = initFromEnv();
        const program = await Program.create<{0}>(IDL, provider.publicKey);

        // ...

    }} catch (e) {{
        console.error(e);
    }}
}});
"#,
        name.to_camel_case(),
        name.to_mixed_case()
    )
}

pub fn main_dot_go() -> String {
    r#"package main

import (
    . "solana"
)

// This is the definition of the IxInit Instruction
type IxInit struct {
    // First, list all the accounts that are used by the instruction
    // Use tags to specify the account attributes:
    // - `account:"signer"` for the accounts that are used as signer
    // - `account:"mut"` for the accounts that are used as writable
    // - `account:"mut, signer` for the accounts that are used as signer and writable
    // If you need to access the account data, add the `data:"accountData"` tag to the field
    // where `accountData` is a type name you defined in this package
    // ...

    // Then, list all the programs that are used by the instruction
    // programs are a kind of accounts, but they usually don't get directly referenced in 
    // your code. All the programs required by the APIs you use must be listed here.
    // ...

    // Finally, list all the instruction parameters
    // ...
}



// This is the business logic of the IxInit
func (ix *IxInit) Process() {
    // ...
}

type IxBusiness struct {
    // ...
}

func (ix *IxBusiness) Process() {
    
}

// This is the entry point of the program
func main() {
    GetIx().Process()
} 
"#
    .to_owned()
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

pub fn gitignore() -> String {
    r#"target
node_modules"#
        .to_owned()
}

pub fn eslintrc() -> String {
    r#"{
    "root": true,
    "parser": "@typescript-eslint/parser",

    "plugins": [
        "@typescript-eslint"
    ],

    "extends": [
        "plugin:@typescript-eslint/recommended"
    ],

    "env": {
        "browser": true,
        "node": true
    }
}
"#
    .to_owned()
}

pub fn mocharc() -> String {
    r#"module.exports = {
    extension: ['js', 'jsx', 'ts', 'tsx'],
    spec: ['tests/**.{js,ts,jsx,tsx}'],
    loader: 'ts-node/esm'
};"#
    .to_owned()
}

pub fn tsconfig() -> String {
    r#"{
    "compilerOptions": {
        /* Visit https://aka.ms/tsconfig to read more about this file */
        "target": "es2020",
        "module": "node16",
        "moduleResolution": "node16",
        "esModuleInterop": true,
        "forceConsistentCasingInFileNames": true,
        "strict": true,
        "skipLibCheck": true
    }
}"#
    .to_owned()
}

pub fn npm_package() -> String {
    r#"{
        "type": "module",
        "scripts": {
            "lint:fix": "prettier */*.js \"*/**/*{.js,.ts}\" -w",
            "lint": "prettier */*.js \"*/**/*{.js,.ts}\" --check"
        },
        "dependencies": {
            "@project-serum/anchor": "^0.26.0",
            "@solana/web3.js": "^1.78.4",
            "bn.js": "^5.2.1",
            "golana": "^0.2.10"
        },
        "devDependencies": {
            "@types/bn.js": "^5.1.1",
            "@types/chai": "^4.3.0",
            "@types/mocha": "^9.0.0",
            "@types/node": "^18.14.6",
            "@typescript-eslint/eslint-plugin": "^5.54.0",
            "@typescript-eslint/parser": "^5.54.0",
            "chai": "^4.3.4",
            "eslint": "^8.35.0",
            "mocha": "^9.0.3",
            "prettier": "^2.6.2",
            "ts-mocha": "^10.0.0",
            "ts-node": "^10.9.1",
            "typescript": "^4.3.5"
        }
    }"#
    .to_owned()
}
