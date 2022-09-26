# Why Golana

Golana will make it possible to run Go code on Solana by deploying a Goscript VM on it. This is not an original idea, [Neon](https://neon-labs.org/) does the same thing for Solidity.

Obviously, Neon has a good reason to pay the price of running a VM on top of Sealevel VM, so why Golana? Simplicity and Verifiable.

## Simplicity

Most programmers would agree that working on Solana is harder than on EVM chains, the reason is obvious: Rust's steep learning curve.

I agree that forcing people to learn Rust is not necessarily a bad thing, but an easier language would definitely help to make Solana more popular among programmers. [Seahorse](https://seahorse-lang.org/) shows this is what people want.

While Seahorse is an exciting and useful project, it's not a clean solution. Correct me if I'm wrong, but Rust's compiler is so much more restrictive than Python's, with complex enough code, you'd probably have to know Rust to write Seahorse code that can be translated to valid Rust code.

Go is one of the most popular languages, it's simple yet feature-rich. With Golana you'd have a full-featured Go at your disposal, except for goroutine and channel, which don't make sense on chain anyway.

## Verifiable

One of smart contracts' problem is that they are not in plain text on chain, making it hard to verify. With Golana the source code of the smart contracts is stored on chain for anyone to verify, and you can be sure what you see is what you signed for.

From a user's perspective, it would feel like the source code gets read and run by the on chain VM every time a transaction is confirmed. In reality it would work like this:

- The developer stores the Go code `hello.go` in an account.
- The developer tell the on chain Goscript engine to compile the source code into byte code `hello.bc` and store it in a PDA.
- The user sends a tx telling the on chain Goscript engine to run `hello.go`, the engine read the coresponding `hello.bc` and run it.

As long as the on chain Goscript engine is trusted, you can be sure the code executed is the source code you read.

## Slow? Expensive?

Similar to Neon, it will cost much more lamports, but it'll still be very cheap. As for the confirmation time, users should not feel any difference.

Golana won't compete against Rust solutions, it'll be for Go programmers who don't want to learn Rust or those who want true verification.
