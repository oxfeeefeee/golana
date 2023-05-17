# Solana as a serverless platform

## Serverless

"Serverless" is a cloud computing model in which the cloud provider manages the server, and the user is only responsible for providing the code. Users are charged only for the time that the code is running, and the code can be scaled automatically.

By definition, Solana is a serverless platform, and each Solana instruction is a serverless function, also known as "Function as a Service" (FaaS).

When you develop a smart contract on Solana, you are actually developing a set of serverless functions, referred to as instructions. These functions are then compiled into eBPF bytecode and deployed onto the serverless platform - the Solana chain.

## Solana smart contract

A Solana smart contract is called a "program," and it consists of a set of instructions. Each instruction is a serverless function. Let's explore how we can implement these functions to perform tasks such as DeFi.

Let's take a look at the "hello wrold" example:

```rust
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo], // accounts that the program can access
    instruction_data: &[u8] // arguments passed to the program by the caller, a client or another instruction
) -> ProgramResult {
    // log a message to the blockchain
    msg!("Hello, world!");
    Ok(())
}
```

What do you typically do in this function? There isn't much difference between developing a Web 2.0 serverless function and a Solana instruction:

- Authenticating users vs. Verifying signatures
- Accessing the database vs. Accessing accounts
- Implementing business logic vs. Implementing business logic
- Calling other functions/services vs. Calling other instructions

There are two significant differences compared to developing web2 serverless functions:

- Since it's a public and trustless environment, a significant amount of effort and code is dedicated to ensuring security. The program must confirm that the correct user has signed the transaction and can only modify accounts that the user has permission to access.

- There is no built-in database, and data must be dealt with in the form of raw bytes. Whatever needs to be stored or retrieved must be encoded and decoded manually, similar to working with a file.

## Programming frameworks

Currently, there are two programming frameworks available for Solana: [The offical SDK](https://docs.solana.com/developing/on-chain-programs/developing-rust) and [Anchor](https://www.anchor-lang.com/), both are for Rust developers. Most developers choose Anchor, which is built on top of the official SDK, because it handles a lot of the boilerplate work and security checks for you.

Golana is aiming to become the third programming framework for Solana, and the first one for Go programmers.
