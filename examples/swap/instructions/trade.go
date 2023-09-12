package instructions

import (
	"math2"
	. "solana"
	"token"
)

type IxTrade struct {
	// The trader, i.e. the liquidity provider
	trader Account `account:"mut, signer"`
	// trader's token A/B account
	tokenA Account `account:"mut"`
	tokenB Account `account:"mut"`
	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault    Account `account:"mut"`
	tokenBVault    Account `account:"mut"`
	vaultAuthority Account
	// The pool account storing the pool data
	poolInfo Account `data:"poolData"`

	tokenProgram Program

	// Trade offeredAmount A for at least expectedAmount B
	offeredAmount  uint64
	expectedAmount uint64
	// If inverse is true, then the trade is offeredAmount B for at least expectedAmount A
	inverse       bool
	vaultAuthBump uint8
}

func (ix *IxTrade) Process() {
	data := ix.poolInfo.Data().(*poolData)
	Assert(data.tokenAVault == *ix.tokenAVault.Key(), "")
	Assert(data.tokenBVault == *ix.tokenBVault.Key(), "")

	vaultA, err := token.UnpackAccount(ix.tokenAVault)
	AbortOnError(err)
	vaultB, err := token.UnpackAccount(ix.tokenBVault)
	AbortOnError(err)
	liquidity := math2.U64GeometryMean(vaultA.Amount, vaultB.Amount)

	var leftToken, rightToken, leftVault, rightVault Account
	var leftAmount, rightAmount uint64
	if !ix.inverse {
		leftToken, rightToken = ix.tokenA, ix.tokenB
		leftVault, rightVault = ix.tokenAVault, ix.tokenBVault
		leftAmount, rightAmount = vaultA.Amount, vaultB.Amount
	} else {
		leftToken, rightToken = ix.tokenB, ix.tokenA
		leftVault, rightVault = ix.tokenBVault, ix.tokenAVault
		leftAmount, rightAmount = vaultB.Amount, vaultA.Amount
	}
	newRightAmount := math2.U64MulDiv(liquidity, liquidity, leftAmount+ix.offeredAmount)
	toTransfer := rightAmount - newRightAmount
	if toTransfer < ix.expectedAmount {
		panic("Cannot swap for the expected amount")
	}

	vaultAuthSeedBump := []SeedBump{{VAULT_AUTH_PDA_SEED, ix.vaultAuthBump}}
	// Transfer token A to the pool
	AbortOnError(token.Transfer(
		leftToken,
		leftVault,
		ix.trader,
		ix.offeredAmount,
		nil,
	))
	AbortOnError(token.Transfer(
		rightVault,
		rightToken,
		ix.vaultAuthority,
		toTransfer,
		vaultAuthSeedBump,
	))
}
