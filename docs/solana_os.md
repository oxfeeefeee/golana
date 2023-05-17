# Solana as an OS

## Confusing Terminology

In my opinion, Solana did a poor job naming its terms - your account is not account and your owner is not owner.  If you want to understand Solana from a smart contract developer's point of view and the official documentation has failed you, give this document a try.

First, forget everything you know about Solana, as the terms you will encounter below have nothing to do with the official terms.

## The Solana OS

Think of Solana as an operating system, like Windows or Linux but much simpler. Solana OS is all about files, it manages files and it runs executable files.

Each file corresponds to a public-private key pair (with one exception, which we will explain later). The public key serves as the file path, while the private key should never be seen by the OS. The OS only sees the signature of the private key, which is provided by whoever holds the private key to authorize some action.

For example, your wallet is a file that records the balance of a particular token. You can sign with your private key to authorize a deduction from your balance. An executable file on the chain must perform the deduction on your behalf, with your authorization. We call this executable file a program.

Just like in Windows, every file on Solana needs a program to be opened. You must assign only one program to a file, and the program is the only one that can read and write the file. That's why the official documents call the program the "owner" of the file.

Every file has a built-in SOL balance. The operating system collects rent from the balance for storing the file, and the file will be deleted when the balance reaches zero. If you deposit more than two years' worth of rent, the file is exempt from rent, so most people choose to do that.

Solana OS Q&A:

- What is the executable file that opens other executable files?

  The system program.

- How do I create a file?

  By calling the system program.

- Can I change the program that opens a file?

  Only when the file is empty.

## The PDA

As mentioned earlier, every file on Solana requires an off-chain user holding the private key. But what if a program just needs to create a temporary file to store data? The program cannot hold private keys, so this is where PDA comes in.

A PDA file is a file without a private key, and its path (the public key) is just a hash of the program's public key combined with a unique string. The program can then act as if it holds the private key of the file.

It's worth noting that for a PDA file, the program that holds the private key and the program that opens the file do not have to be the same. The program that holds the private key can sign on its behalf, and the program that opens the file can access it.

## Wrap-up

These are the core concepts you need to understand to develop on Solana, which was also the most confusing part for me. I hope this helps.

As you may already know, the files we've been discussing are officially called "accounts".
