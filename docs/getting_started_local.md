# Getting started with local cluster

## TLDR

- Install Anchor following this: <https://www.anchor-lang.com/docs/installation>

- Clone Golana repo: <https://github.com/oxfeeefeee/golana>

- Start your local Solana validator, in your home directory (or any other dir) run:

    ```bash
    solana-test-validator
    ```

- Deploy the Golana loader

    1. Go to `GOLANA_REPO/loader/`

    2. Build and deploy

        ```bash
        anchor build
        anchor deploy
        ```

        Record the the final string printed by `anchor deploy`, which is the loader's program id

- Compile the Golana cli

    1. Go to `GOLANA_REPO/cli/`

    2. Run `cargo build` which will output the binary to `GOLANA_REPO/target/debug/golana-cli`

- Run the helloworld example

    1. Goto `GOLANA_REPO/examples/helloworld/`

    2. Open Golana.toml, replace the loader_id with the id you just got

    3. Deploy `helloworld`, run:

        ```bash
        ../../target/debug/golana-cli deploy
        ```

    4. Install Node libraries for testing, run:

        ```bash
        npm install
        ```

    5. Run the test:

        ```bash
        ../../target/debug/golana-cli test
        ```

    You should see some green if everything goes OK.

- Create your first Golana program

    1. Assuming you are using `GOLANA_REPO/examples/` as your working directory, go to `GOLANA_REPO/examples/`

    2. Init a project:

        ```bash
        ../target/debug/golana-cli init --name=to_the_sun
        ```

    3. Now you have a golana project in `GOLANA_REPO/examples/to_the_sun/`, compare it with `GOLANA_REPO/examples/helloworld` and start coding!

## Explanations

It's possible to get started with devnet/testnet, but it's always easier to test your program with a local cluster, as you have minimal latency and unlimited money. However, it takes a few extra steps to set it up.

Golana programs are just GoScript bytecode. In order to run them, we need a standard Solana program that embeds a GoScript VM to load it, which is the loader.

The loader itself is built with Anchor, and it does not come with the official validator node, so you have to deploy it yourself to your local node. That's why you need to install Anchor.
