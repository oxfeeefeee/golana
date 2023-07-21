package instructions

import (
	"math2"
	. "solana"
	"token"
)

type IxWithdraw struct {
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

	tokenProgram *AccountInfo

	// The amount of token A/B to deposit
	amount        uint64
	vaultAuthBump uint8
}

func (ix *IxWithdraw) Process() {
	liq, err := token.UnpackAccount(ix.tokenLiquidity)
	AbortOnError(err)
	if liq.Amount < ix.amount {
		panic("Not enough liquidity token")
	}
	liqMint, err := token.UnpackMint(ix.mintLiquidity)
	AbortOnError(err)
	vaultA, err := token.UnpackAccount(ix.tokenAVault)
	AbortOnError(err)
	vaultB, err := token.UnpackAccount(ix.tokenBVault)
	AbortOnError(err)

	amountA := math2.U64MulDiv(vaultA.Amount, ix.amount, liqMint.Supply)
	amountB := math2.U64MulDiv(vaultB.Amount, ix.amount, liqMint.Supply)

	// Transfer token A/B to the pool
	vaultAuthSeedBump := []SeedBump{{VAULT_AUTH_PDA_SEED, ix.vaultAuthBump}}
	AbortOnError(token.Transfer(
		ix.tokenAVault,
		ix.tokenA,
		ix.vaultAuthority,
		amountA,
		vaultAuthSeedBump,
	))
	AbortOnError(token.Transfer(
		ix.tokenBVault,
		ix.tokenB,
		ix.vaultAuthority,
		amountB,
		vaultAuthSeedBump,
	))

	// Burn the liquidity token from the depositor's account
	AbortOnError(token.Burn(
		ix.tokenLiquidity,
		ix.mintLiquidity,
		ix.depositor,
		ix.amount,
		nil,
	))

}
