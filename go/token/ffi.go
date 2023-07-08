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
	unpack_mint(account *AccountInfo) (*Mint, unsafe.Pointer)

	create_and_init_account(from, to, mint uint, owner *PublicKey, signerSeeds []SeedBump) unsafe.Pointer

	close_account(account, dest, owner uint, signerSeeds []SeedBump) unsafe.Pointer

	set_authority(accountOrMint, currentAuth uint, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) unsafe.Pointer

	transfer(from, to, auth uint, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	mint_to(mint, dest, auth uint, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	burn(account, mint, auth uint, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	create_associated_account(from, to, owner, mint, sys, spl uint, idempotent bool) unsafe.Pointer
}
