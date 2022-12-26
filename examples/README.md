# How to run a golana program

## Fire up a local validator

- Open a new terminal and `solana-test-validator`
- Optionally open an other new terminal and `solana logs`

## Deploy the loader(An anchor program that contains the Goscript VM)

- Open a new terminal and go to Golana/loader
- Run `anchor build`
- Run `anchor deploy`

## Compile the golana program, take `examples/escrow` as an example

- First we need to compile `cli`, the goscript compiler. Open a new terminal and go to Golana/cli.
- Run `cargo build`
- Go to Golana/examples/escrow, run `../../target/debug/golana-cli build`

## Run the test script, it's located at `Golana/loader/tests/loader.ts` for now

- Run `anchor test --skip-build --skip-deploy --skip-local-validator`, the script will deploy the golana binary and run the transactions.
