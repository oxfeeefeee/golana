# Milestones

## One: Prototype

- An escrow program like [this](https://github.com/paul-schaaf/solana-escrow) or [this](https://github.com/ironaddicteddog/anchor-escrow) to prove the idea is viable.

- The program is deployed to the testnet and passes basic functionality tests. It may be buggy and does not have properly designed APIs nor documentation.

## Two: Alpha Version

- An Anchor-lang like framework with all the features to support an escrow program including:

  - Basic Solana DS and API exposed to Goscript, like PubKey, AccountInfo, token::TokenAccount, token::SetAuthority, etc.
  - Automatic serialize and deserialize Account data
  - Automatic serialize and deserialize args of processors
  - Automatic account attributes checks.
  
  basically try to make Anchor users feel at home.

- A Golana version of escrow program deployed to testnet and it passes basic functionality tests.

- There might be known bugs. No documentation other than code comments. No client-side support.

## Three: Beta Version

- All known major bugs are fixed. A few more APIs may be added if necessary.

- A CLI tool like Anchor's, and the clients generation code borrowed from Anchor is integrated.

- A basic curve-based-DEX demo is developed with Golana (maybe without GUI) as an example.

- A website for Golana to show basic documentations of how to use Golana.

- Golana deployed to mainnet, a toy-tool on the website to edit, deploy and interact with "hello_world.go" on the mainnet.
