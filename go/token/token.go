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

type Mint struct {
	MintAuthority   *PublicKey
	Supply          uint64
	Decimals        uint8
	IsInitialized   bool
	FreezeAuthority *PublicKey
}

func UnpackMint(account *AccountInfo) (*Mint, error) {
	mint, err := tokenFfi.unpack_mint(account)
	return mint, NewSolanaError(err)
}

func CreateAndInitAccount(from, to, mint *AccountInfo, owner *PublicKey, signerSeeds []SeedBump) error {
	err := tokenFfi.create_and_init_account(from.Index, to.Index, mint.Index, owner, signerSeeds)
	return NewSolanaError(err)
}

func CloseAccount(account, dest, owner *AccountInfo, signerSeeds []SeedBump) error {
	err := tokenFfi.close_account(account.Index, dest.Index, owner.Index, signerSeeds)
	return NewSolanaError(err)
}

func SetAuthority(accountOrMint, currentAuth *AccountInfo, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) error {
	err := tokenFfi.set_authority(accountOrMint.Index, currentAuth.Index, newAuth, authType, signerSeeds)
	return NewSolanaError(err)
}

func Transfer(from, to, auth *AccountInfo, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.transfer(from.Index, to.Index, auth.Index, amount, signerSeeds)
	return NewSolanaError(err)
}

func MintTo(mint, dest, auth *AccountInfo, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.mint_to(mint.Index, dest.Index, auth.Index, amount, signerSeeds)
	return NewSolanaError(err)
}

func Burn(account, mint, auth *AccountInfo, amount uint64, signerSeeds []SeedBump) error {
	err := tokenFfi.burn(account.Index, mint.Index, auth.Index, amount, signerSeeds)
	return NewSolanaError(err)
}

func CreateAssociatedAccount(from, to, owner, mint, sys, spl *AccountInfo, idempotent bool) error {
	err := tokenFfi.create_associated_account(from.Index, to.Index, owner.Index, mint.Index, sys.Index, spl.Index, idempotent)
	return NewSolanaError(err)
}
