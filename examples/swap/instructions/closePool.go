package instructions

import (
	. "solana"
	"token"
)

type IxClosePool struct {
	// The creator of the pool, i.e. the one who called IxClosePool
	creator Account `account:"mut, signer"`
	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault    Account `account:"mut"`
	tokenBVault    Account `account:"mut"`
	vaultAuthority Account
	// The pool account storing the pool data
	poolInfo Account `account:"mut"`

	systemProgram Program
	tokenProgram  Program

	authBump uint8
}

func (ix *IxClosePool) Process() {
	authSeedBump := []SeedBump{{VAULT_AUTH_PDA_SEED, ix.authBump}}

	AbortOnError(token.CloseAccount(
		ix.tokenAVault,
		ix.creator,
		ix.vaultAuthority,
		authSeedBump))
	AbortOnError(token.CloseAccount(
		ix.tokenBVault,
		ix.creator,
		ix.vaultAuthority,
		authSeedBump))
}
