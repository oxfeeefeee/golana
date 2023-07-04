package instructions

import (
	. "solana"
)

type IxDeposit struct {
	// The depositor, i.e. the liquidity provider
	depositor *AccountInfo `golana:"mut, signer"`
	// The mint of the liquidity token
	mintLiquidity *AccountInfo
	// depositor's token A/B account
	tokenA *AccountInfo `golana:"mut"`
	tokenB *AccountInfo `golana:"mut"`
	// depositor's liqudity token account, i.e the assosicated account
	tokenLiquidity *AccountInfo `golana:"mut"`

	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault    *AccountInfo
	tokenBVault    *AccountInfo
	vaultAuthority *AccountInfo
	// The pool account storing the pool data
	poolInfo *AccountInfo `golana:"mut"`

	systemProgram *AccountInfo
	tokenProgram  *AccountInfo
	rent          *AccountInfo

	// The amount of token A/B to deposit
	amountA  uint64
	amountB  uint64
	authBump uint8
}

func (ix *IxDeposit) Process() {
	// Create the liquidity token account as associated account if not exists
	AbortOnError(TokenCreateAssociatedAccount(
		ix.depositor,
		ix.tokenLiquidity,
		ix.depositor,
		ix.mintLiquidity,
		ix.systemProgram,
		ix.tokenProgram,
		true,
	))
}
