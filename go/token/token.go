package token

import (
	. "solana"
)

func CreateAndInitAccount(from, to, mint *AccountInfo, owner *PublicKey, signerSeeds []SeedBump) error {
	p := tokenFfi.token_create_and_init_account(from.Index, to.Index, mint.Index, owner, signerSeeds)
	return NewSolanaError(p)
}

func CloseAccount(account, dest, owner *AccountInfo, signerSeeds []SeedBump) error {
	p := tokenFfi.token_close_account(account.Index, dest.Index, owner.Index, signerSeeds)
	return NewSolanaError(p)
}

func SetAuthority(accountOrMint, currentAuth *AccountInfo, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) error {
	p := tokenFfi.token_set_authority(accountOrMint.Index, currentAuth.Index, newAuth, authType, signerSeeds)
	return NewSolanaError(p)
}

func Transfer(from, to, auth *AccountInfo, amount uint64, signerSeeds []SeedBump) error {
	p := tokenFfi.token_transfer(from.Index, to.Index, auth.Index, amount, signerSeeds)
	return NewSolanaError(p)
}

func MintTo(mint, dest, auth *AccountInfo, amount uint64, signerSeeds []SeedBump) error {
	p := tokenFfi.token_mint_to(mint.Index, dest.Index, auth.Index, amount, signerSeeds)
	return NewSolanaError(p)
}

func Burn(account, mint, auth *AccountInfo, amount uint64, signerSeeds []SeedBump) error {
	p := tokenFfi.token_burn(account.Index, mint.Index, auth.Index, amount, signerSeeds)
	return NewSolanaError(p)
}

func CreateAssociatedAccount(from, to, owner, mint, sys, spl *AccountInfo, idempotent bool) error {
	p := tokenFfi.token_create_associated_account(from.Index, to.Index, owner.Index, mint.Index, sys.Index, spl.Index, idempotent)
	return NewSolanaError(p)
}
