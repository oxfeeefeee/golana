---
title: Installation
pageTitle: Golana - Installation
---

## Install CLI

- Install [Cargo(Rust)](https://doc.rust-lang.org/cargo/getting-started/installation.html). (This is just for installing Golana CLI)

- Install Golana CLI:
  ```bash
  cargo install golana-cli
  ```

## Your first Golana project

- Create a new project

  ```bash
  golana init --name=my_proj
  ```

  This creates a new project for you with the built-in template

- Complete your project

  Compare `main.go` and `tests/my_proj.ts` with corresponding files in the [Hellworld example](https://github.com/oxfeeefeee/golana/tree/main/examples/helloworld), and make some proper changes

- Compile and deploy your project to testnet

  In the newly created my_proj directory:

  ```bash
  golana deploy -rf   # rf means re-compile & force
  ```

- Test your project

  - Install Node if you haven't.
  - ```bash
    npm install
    ```
  - ```bash
    golana test
    ```

## Local development

- It's faster to get started with devnet/testnet, but it's always easier to test your program with a local cluster, as you have minimal latency and unlimited money. However, it takes a few extra steps to set it up.

- Install Anchor following this: <https://www.anchor-lang.com/docs/installation>

- Clone Golana repo: <https://github.com/oxfeeefeee/golana>

- Start your local Solana validator, in your home directory (or any other dir) run:

  ```bash
  solana-test-validator
  ```

- Deploy the Golana loader

  - Go to `GOLANA_REPO/loader/`

  - Build and deploy

    ```bash
    anchor build
    anchor deploy
    ```

    Record the the final string printed by `anchor deploy`, which is the loader's program id.

    **NOTE**: Don't forget to change all the loader_id in the Golana.toml files

- Optionally, compile your own Golana cli

  - Go to `GOLANA_REPO/cli/`

  - Run `cargo build` which will output the binary to `GOLANA_REPO/target/debug/golana-cli`

## Notes

- Golana programs are just GoScript bytecode. In order to run them, we need a standard Solana program that embeds a GoScript VM to load it, which is the loader. The loader itself is built with Anchor, and it does not come with the official validator node, so you have to deploy it yourself to your local node. That's why you need to install Anchor.

- In theory we should be using devnet, but it seems it's very easy to get rate limited when airdropping SOL on devnet

- The memory and CPU limit are always set to the maximum by the Golana client code, which is because running a complex VM as a Solana program is pretty expensive.
