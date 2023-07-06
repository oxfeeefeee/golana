package instructions

import (
	. "solana"
	"token"
)

type IxDeposit struct {
	// The depositor, i.e. the liquidity provider
	depositor *AccountInfo `golana:"mut, signer"`
	// The mint of the liquidity token
	mintLiquidity *AccountInfo `golana:"mut"`
	mintLpAuth    *AccountInfo
	// depositor's token A/B account
	tokenA *AccountInfo `golana:"mut"`
	tokenB *AccountInfo `golana:"mut"`
	// depositor's liqudity token account, i.e the assosicated account
	tokenLiquidity *AccountInfo `golana:"mut"`

	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault    *AccountInfo `golana:"mut"`
	tokenBVault    *AccountInfo `golana:"mut"`
	vaultAuthority *AccountInfo
	// The pool account storing the pool data
	poolInfo *AccountInfo `golana:"mut"`

	systemProgram          *AccountInfo
	tokenProgram           *AccountInfo
	associatedTokenProgram *AccountInfo
	rent                   *AccountInfo

	// The amount of token A/B to deposit
	amountA      uint64
	amountB      uint64
	mintAuthBump uint8
}

func (ix *IxDeposit) Process() {
	// Create the liquidity token account as associated account if not exists
	AbortOnError(token.CreateAssociatedAccount(
		ix.depositor,
		ix.tokenLiquidity,
		ix.depositor,
		ix.mintLiquidity,
		ix.systemProgram,
		ix.associatedTokenProgram,
		true,
	))

	// Transfer token A/B to the pool
	AbortOnError(token.Transfer(
		ix.tokenA,
		ix.tokenAVault,
		ix.depositor,
		ix.amountA,
		nil,
	))
	AbortOnError(token.Transfer(
		ix.tokenB,
		ix.tokenBVault,
		ix.depositor,
		ix.amountB,
		nil,
	))

	// Mint the liquidity token to the depositor
	mintAuthSeedBump := []SeedBump{{LP_MINT_AUTH_PDA_SEED, ix.mintAuthBump}}
	AbortOnError(token.MintTo(
		ix.mintLiquidity,
		ix.tokenLiquidity,
		ix.mintLpAuth,
		ix.amountA+ix.amountB,
		mintAuthSeedBump,
	))

}
