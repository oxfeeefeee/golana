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

type Mint struct {
	MintAuthority   *PublicKey
	Supply          uint64
	Decimals        uint8
	IsInitialized   bool
	FreezeAuthority *PublicKey
}

type TokenAccount struct {
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

func UnpackMint(account Account) (*Mint, error) {
	mint, err := tokenFfi.unpack_mint(account)
	return mint, NewSolanaError(err)
}

func UnpackAccount(account Account) (*TokenAccount, error) {
	acc, err := tokenFfi.unpack_account(account)
	return acc, NewSolanaError(err)
}

func CreateAndInitAccount(from, to, mint Account, owner *PublicKey, signerSeeds []SeedBump) error {
	err := tokenFfi.create_and_init_account(from, to, mint, owner, signerSeeds)
	return NewSolanaError(err)
}

func CloseAccount(account, dest, owner Account, signerSeeds []SeedBump) error {
	err := tokenFfi.close_account(account, dest, owner, signerSeeds)
	return NewSolanaError(err)
}

func SetAuthority(accountOrMint, currentAuth Account, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) error {
	err := tokenFfi.set_authority(accountOrMint, currentAuth, newAuth, authType, signerSeeds)
	return NewSolanaError(err)
}

func Transfer(from, to, auth Account, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.transfer(from, to, auth, amount, signerSeeds)
	return NewSolanaError(err)
}

func MintTo(mint, dest, auth Account, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.mint_to(mint, dest, auth, amount, signerSeeds)
	return NewSolanaError(err)
}

func Burn(account, mint, auth Account, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.burn(account, mint, auth, amount, signerSeeds)
	return NewSolanaError(err)
}

func CreateAssociatedAccount(from, to, owner, mint, sys, spl Account, idempotent bool) error {
	err := tokenFfi.create_associated_account(from, to, owner, mint, sys, spl, idempotent)
	return NewSolanaError(err)
}
