# Programming with Golana

## Introduction

A Solana project is composed of two main components: the on-chain program and the client. Generally, the client requires more code than the on-chain program. The client is responsible for user interaction, wallet management, and transaction submission to the chain, and the on-chain program handles transaction processing and updates the data stored on-chain.

This document focuses on the on-chain component.

## The entry point

Every Golana program is a `main.go` file. It contains the `main` function, which is the entry point of the program. The `main` function is responsible for getting the current instruction, which is set up by the runtime, and calling `Process()` on it. Every instruction is a struct that implements the `Instruction` interface, which has a `Process()` function.

```go
func main() {
    GetIx().Process()
}
```

## Implementing instructions

As previously mentioned, each instruction is a struct with a Process() function, which serves as the instruction's entry point.

Within the struct, there are three categories of fields:

- Accounts - The accounts that the instruction can access. These accounts are provided to the instruction by the client.

- Account Data - To read or write data from any accounts, you must declare the data type here.

- Arguments - The arguments sent to the instruction by the client.

These fields must be declared in the order of accounts, account data, and arguments; otherwise, the compiler will generate an error.

## Example

Let's take a look at the greeting (aka "hello world") example, which is included in the [examples](https://github.com/oxfeeefeee/golana/tree/main/examples) folder. It contains two instructions: `IxInit` and `IxGreet`. `IxInit` stores the pub-key of the user and init the counter to a value specified by the user. `IxGreet` increments the counter and prints a greeting message, only the original user can call this instruction.

Here is the code of `IxInit`:

```go
type IxInit struct {
    // First, list all the accounts that are used by the instruction
    // Use tags to specify the account attributes:
    // - `golana:"signer"` for the accounts that are used as signer
    // - `golana:"mut"` for the accounts that are used as writable
    user        *AccountInfo `golana:"signer"`
    userAccount *AccountInfo `golana:"mut"`

    // Second, declare the data stored in the accounts, that needs to be read or written by the instruction
    // Use the corresponding account name with a _data suffix,
    // and add the `golana:"init"` or `golana:"mut"` tag to the field:
    // - `golana:"init"` for the data that will be initialized by the instruction
    // - `golana:"mut"` for the data that will be written by the instruction
    userAccount_data *UserData `golana:"init"`

    // Finally, list all the instruction parameters
    initialCount uint64
}

type UserData struct {
    auth       PublicKey
    greetCount uint64
}

// This is the business logic of the IxInit
func (ix *IxInit) Process() {
    data := new(UserData)
    // set the auth of userAccount as the user
    data.auth = *ix.user.Key
    data.greetCount = ix.initialCount
    ix.userAccount_data = data
    CommitData(ix.userAccount)
}
```

There should be enough comments in the code to explain what's going on. One thing worth noting is that the CommitData() function, which is different from other frameworks. With other frameworks, the data is written back to the account implicitly. With Golana, you need to call CommitData() explicitly. This is because Golana runs on the Goscript VM, there is another layer of abstraction.

We'll not go through the code of `IxGreet` here, for it's very similar to `IxInit` and should be self-explanatory.

## The solana module

Every Golana program needs to import the `solana` module, which provides the interfaces to interact with the Solana runtime. The Go part of the code is here: [solana](https://github.com/oxfeeefeee/golana/tree/main/go/solana), and the Rust part is here: [solana.rs](https://github.com/oxfeeefeee/golana/blob/main/loader/programs/loader/src/ffi/solana.rs), in case you what to take a look at the implementation.

## The compiler

When you execute `golana-cli build`, it performs three tasks:

- Compiles the Go code into Goscript Bytecode.

- Ensures the Go program complies with Golana rules.

- Generates an IDL, similar to Anchor, to enable the client to conveniently call the instructions.
