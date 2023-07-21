package instructions

import (
	. "solana"
	"token"
)

type IxTrade struct {
	// The trader, i.e. the liquidity provider
	trader *AccountInfo `golana:"mut, signer"`
	// trader's token A/B account
	tokenA *AccountInfo `golana:"mut"`
	tokenB *AccountInfo `golana:"mut"`
	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault    *AccountInfo `golana:"mut"`
	tokenBVault    *AccountInfo `golana:"mut"`
	vaultAuthority *AccountInfo
	// The pool account storing the pool data
	poolInfo *AccountInfo `golana:"mut"`

	tokenProgram *AccountInfo

	// Trade amountA for at least expectedAmountB
	amountA         uint64
	expectedAmountB uint64
	vaultAuthBump   uint8
}

func (ix *IxTrade) Process() {

	vaultAuthSeedBump := []SeedBump{{VAULT_AUTH_PDA_SEED, ix.vaultAuthBump}}
	// Transfer token A to the pool
	AbortOnError(token.Transfer(
		ix.tokenA,
		ix.tokenAVault,
		ix.trader,
		ix.amountA,
		nil,
	))
	AbortOnError(token.Transfer(
		ix.tokenBVault,
		ix.tokenB,
		ix.vaultAuthority,
		ix.amountB,
		vaultAuthSeedBump,
	))
}
