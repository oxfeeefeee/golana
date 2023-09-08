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

	unpack_account(account Account) (*Token, unsafe.Pointer)

	create_and_init_account(payer, to, mint Account, wallet *PublicKey, signerSeeds []SeedBump) unsafe.Pointer

	close_account(account, dest, wallet Account, signerSeeds []SeedBump) unsafe.Pointer

	set_authority(accountOrMint, currentAuth Account, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) unsafe.Pointer

	transfer(from, to, auth Account, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	mint_to(mint, dest, auth Account, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	burn(account, mint, auth Account, amount uint64, signerSeeds []SeedBump) unsafe.Pointer

	create_associated_account(payer, to, wallet, mint Account, sys, tp Program, idempotent bool, signerSeeds []SeedBump) unsafe.Pointer
}
