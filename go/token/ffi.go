package token

import (
	. "solana"
	"unsafe"
)

var tokenFfi ffiToken

func init() {
	tokenFfi = ffi(ffiToken, "token")
}

type ffiToken interface {
	unpack_mint(account Account) (*Mint, unsafe.Pointer)

	unpack_account(account Account) (*TokenAccount, unsafe.Pointer)

	create_and_init_account(from, to, mint Account, owner *PublicKey, signerSeeds []SeedBump) unsafe.Pointer

	close_account(account, dest, owner Account, signerSeeds []SeedBump) unsafe.Pointer

	set_authority(accountOrMint, currentAuth Account, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) unsafe.Pointer

	transfer(from, to, auth Account, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	mint_to(mint, dest, auth Account, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	burn(account, mint, auth Account, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	create_associated_account(from, to, owner, mint, sys, spl Account, idempotent bool) unsafe.Pointer
}
