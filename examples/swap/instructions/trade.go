package instructions

import (
	"math2"
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
	vaultA, err := token.UnpackAccount(ix.tokenAVault)
	AbortOnError(err)
	vaultB, err := token.UnpackAccount(ix.tokenBVault)
	AbortOnError(err)
	liquidity := math2.U64GeometryMean(vaultA.Amount, vaultB.Amount)
	newVaultBAmount := math2.U64MulDiv(liquidity, liquidity, vaultA.Amount+ix.amountA)
	amountB := vaultB.Amount - newVaultBAmount
	if amountB < ix.expectedAmountB {
		panic("Cannot swap for the expected amount")
	}

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
		amountB,
		vaultAuthSeedBump,
	))
}
