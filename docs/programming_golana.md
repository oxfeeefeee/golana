---
title: Programming with Golana
pageTitle: Golana - Programming with Golana
---

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

- Programs - Programs are a kind of accounts, but they may not get directly referenced in your code. All the programs required by the APIs you use must be listed here. They are also provided to the instruction by the client.

- Arguments - The arguments sent to the instruction by the client.

These fields must be declared in the order of accounts, programs, and arguments; otherwise, the compiler will generate an error.

## Example

Let's take a look at the greeting (aka "hello world") example, which is included in the [examples](https://github.com/oxfeeefeee/golana/tree/main/examples) folder. It contains two instructions: `IxInit` and `IxGreet`. `IxInit` stores the pub-key of the user and init the counter to a value specified by the user. `IxGreet` increments the counter and prints a greeting message, only the original user can call this instruction.

Here is the code of `IxInit`:

```go
// This is the definition of the IxInit Instruction
type IxInit struct {
	// First, list all the accounts that are used by the instruction
	// Use tags to specify the account attributes:
	// - `account:"signer"` for the accounts that are used as signer
	// - `account:"mut"` for the accounts that are used as writable
	// - `account:"mut, signer` for the accounts that are used as signer and writable
	// If you need to access the account data, add the `data:"accountData"` tag to the field
	// where `accountData` is a type name you defined in this package

	// The user's "main" account
	user Account `account:"mut, signer"`
	// The account to be created to store the user's data on chain
	userAccount Account `account:"mut, signer" data:"userData"`

	// Then, list all the programs that are used by the instruction
	// programs are a kind of accounts, but they may not get directly referenced in
	// your code. All the programs required by the APIs you use must be listed here.
	// The system program account is used to create the userAccount
	systemProgram Program

	// Finally, list all the instruction parameters
	// Set the initialCount of the greet greater than 0 to cheat
	initialCount uint64
}

type userData struct {
	// Save the Pubkey of the user, so that it won't greet to other users
	auth PublicKey
	// How many times the user has been greeted
	greetCount uint64
}

// This is the business logic of the IxInit
func (ix *IxInit) Process() {
	// On the client side, the userAccount is just a newly generated keypair
	// we now initialize it on chain
	ix.userAccount.Create(ix.user, 512, nil)

	data := new(userData)
	// set the auth of userAccount as the user
	data.auth = *ix.user.Key()
	data.greetCount = ix.initialCount
	ix.userAccount.SaveData(data)
}
```

There should be enough comments in the code to explain what's going on. One thing worth noting is that the SaveData() function, which is different from other frameworks. With other frameworks, the data is written back to the account implicitly. With Golana, you need to call SaveData() explicitly. This is because Golana runs on the Goscript VM, there is another layer of abstraction.

We'll not go through the code of `IxGreet` here, for it's very similar to `IxInit` and should be self-explanatory.

## The solana module

Every Golana program needs to import the `solana` module, which provides the interfaces to interact with the Solana runtime. The Go part of the code is here: [solana](https://github.com/oxfeeefeee/golana/tree/main/cli/go/solana), and the Rust part is here: [solana.rs](https://github.com/oxfeeefeee/golana/blob/main/loader/programs/loader/src/ffi/solana.rs), in case you what to take a look at the implementation.

## The compiler

When you execute `golana build`, it performs three tasks:

- Compiles the Go code into Goscript Bytecode.

- Ensures the Go program complies with Golana rules.

- Generates an IDL, similar to Anchor, to enable the client to conveniently call the instructions.
