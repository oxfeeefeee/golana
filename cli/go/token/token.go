package token

import (
	. "solana"
)

type AuthorityType uint8

const (
	// Authority to mint new tokens
	AuthMintTokens AuthorityType = iota
	// Authority to freeze any account associated with the Mint
	AuthFreezeAccount
	// Owner of a given token account
	AuthAccountOwner
	// Authority to close a token account
	AuthCloseAccount
)

type AccountState uint8

const (
	// Account is not yet initialized
	AccountUninitialized AccountState = iota
	// Account is initialized; the account owner and/or delegate may perform permitted operations
	AccountInitialized
	// Account has been frozen by the Mint freeze authority. Neither the account owner nor
	// the delegate are able to perform operations
	AccountFrozen
)

// The content of a SPL Mint account
type Mint struct {
	MintAuthority   *PublicKey
	Supply          uint64
	Decimals        uint8
	IsInitialized   bool
	FreezeAuthority *PublicKey
}

// The content of a SPL Token account
type Token struct {
	Mint            *PublicKey
	Owner           *PublicKey
	Amount          uint64
	Delegate        *PublicKey
	State           AccountState
	IsNative        bool
	NativeReserve   uint64
	DelegatedAmount uint64
	CloseAuthority  *PublicKey
}

// Unpack a SPL Mint account
func UnpackMint(account Account) (*Mint, error) {
	mint, err := tokenFfi.unpack_mint(account)
	return mint, NewSolanaError(err)
}

// Unpack a SPL Token account
func UnpackAccount(account Account) (*Token, error) {
	acc, err := tokenFfi.unpack_account(account)
	return acc, NewSolanaError(err)
}

// Create a new SPL Token account
// payer (account:"mut, singer"): The account paying for the creation of the new account
// to (account:"mut, singer"): The account to create
// mint: The Mint of the SPL token
// wallet: The owner of the new account
// signerSeeds: used when the singers are PDAs, pass nil if no signer is PDA
// Required Program(s):
//   - SystemProgram
//   - TokenProgram
func CreateAndInitAccount(payer, to, mint Account, wallet *PublicKey, signerSeeds []SeedBump) error {
	err := tokenFfi.create_and_init_account(payer, to, mint, wallet, signerSeeds)
	return NewSolanaError(err)
}

// Close a SPL Token account, the remaining SOL of the closed account will be transferred
// to the destination account, the token balance has to be 0, otherwise returns an error
// account (account:"mut"):  The account to close
// dest (account:"mut"): The account to receive the remaining SOL of the closed account
// wallet (account:"singer"): The owner of the account
// signerSeeds: used when the singers are PDAs, pass nil if no signer is PDA
// Required Program(s):
//   - TokenProgram
func CloseAccount(account, dest, wallet Account, signerSeeds []SeedBump) error {
	err := tokenFfi.close_account(account, dest, wallet, signerSeeds)
	return NewSolanaError(err)
}

// Set a new authority of a SPL Token account or mint
// accountOrMint (account:"mut"): The account or mint to change the authority
// currentAuth (account:"singer") : The current authority of the account or mint
// newAuth: The new authority of the account or mint
// authType: The type of authority to change
// signerSeeds: used when the singers are PDAs, pass nil if no signer is PDA
// Required Program(s):
//   - TokenProgram
func SetAuthority(accountOrMint, currentAuth Account, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) error {
	err := tokenFfi.set_authority(accountOrMint, currentAuth, newAuth, authType, signerSeeds)
	return NewSolanaError(err)
}

// Transfer tokens
// from (account:"mut"): The account to transfer from
// to (account:"mut"): The account to transfer to
// auth (account:"singer"): The authority of the from account
// amount: The amount of tokens to transfer
// signerSeeds: used when the singers are PDAs, pass nil if no signer is PDA
// Required Program(s):
//   - TokenProgram
func Transfer(from, to, auth Account, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.transfer(from, to, auth, amount, signerSeeds)
	return NewSolanaError(err)
}

// Mint new tokens
// mint (account:"mut"): The mint to mint tokens from
// dest (account:"mut"): The account to receive the minted tokens
// auth (account:"singer"): The authority of the mint
// amount: The amount of tokens to mint
// signerSeeds: used when the singers are PDAs, pass nil if no signer is PDA
// Required Program(s):
//   - TokenProgram
func MintTo(mint, dest, auth Account, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.mint_to(mint, dest, auth, amount, signerSeeds)
	return NewSolanaError(err)
}

// Burn tokens
// account (account:"mut"): The account to burn tokens from
// mint (account:"mut"): The mint of the SPL token
// auth (account:"singer"): The authority of the account
// amount: The amount of tokens to burn
// signerSeeds: used when the singers are PDAs, pass nil if no signer is PDA
// Required Program(s):
//   - TokenProgram
func Burn(account, mint, auth Account, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.burn(account, mint, auth, amount, signerSeeds)
	return NewSolanaError(err)
}

// Create an associated SPL token account
// payer (account:"mut, singer"): The account paying for the creation of the new account
// to (account:"mut"): The account to create
// wallet: The owner of the new account
// mint: The mint of the SPL token
// sys: The system program
// tp: The token program
// atp: The associated token program
// idempotent: If true, the transaction will not fail if the account already exists
// signerSeeds: used when the singers are PDAs, pass nil if no signer is PDA
// Required Program(s):
//   - SystemProgram
//   - TokenProgram
//   - AssociatedTokenProgram
func CreateAssociatedAccount(payer, to, wallet, mint Account, sys, tp Program, idempotent bool, signerSeeds []SeedBump) error {
	err := tokenFfi.create_associated_account(payer, to, wallet, mint, sys, tp, idempotent, signerSeeds)
	return NewSolanaError(err)
}
