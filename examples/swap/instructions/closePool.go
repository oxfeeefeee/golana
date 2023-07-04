package instructions

import (
	. "solana"
)

type IxClosePool struct {
	// The creator of the pool, i.e. the one who called IxClosePool
	creator *AccountInfo `golana:"mut, signer"`
	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault    *AccountInfo `golana:"mut"`
	tokenBVault    *AccountInfo `golana:"mut"`
	vaultAuthority *AccountInfo
	// The pool account storing the pool data
	poolInfo      *AccountInfo `golana:"mut"`
	systemProgram *AccountInfo
	tokenProgram  *AccountInfo

	authBump uint8
}

func (ix *IxClosePool) Process() {
	authSeedBump := []SeedBump{{AUTH_PDA_SEED, ix.authBump}}

	AbortOnError(TokenCloseAccount(
		ix.tokenAVault,
		ix.creator,
		ix.vaultAuthority,
		authSeedBump))
	AbortOnError(TokenCloseAccount(
		ix.tokenBVault,
		ix.creator,
		ix.vaultAuthority,
		authSeedBump))
}
