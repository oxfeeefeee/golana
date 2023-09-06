package instructions

import (
	"math2"
	. "solana"
	"token"
)

type IxDeposit struct {
	// The depositor, i.e. the liquidity provider
	depositor Account `account:"mut, signer"`
	// The mint of the liquidity token
	mintLiquidity Account `account:"mut"`
	mintLpAuth    Account
	// depositor's token A/B account
	tokenA Account `account:"mut"`
	tokenB Account `account:"mut"`
	// depositor's liqudity token account, i.e the assosicated account
	tokenLiquidity Account `account:"mut"`

	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault    Account `account:"mut"`
	tokenBVault    Account `account:"mut"`
	vaultAuthority Account
	// The pool account storing the pool data
	poolInfo Account `data:"poolData"`

	systemProgram          Program
	tokenProgram           Program
	associatedTokenProgram Program

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
		ix.tokenProgram,
		true,
		nil,
	))

	// Calculate numbers
	var amountA, amountB, liquidity uint64
	vaultA, err := token.UnpackAccount(ix.tokenAVault)
	AbortOnError(err)
	vaultB, err := token.UnpackAccount(ix.tokenBVault)
	AbortOnError(err)
	if vaultA.Amount == 0 && vaultB.Amount == 0 {
		// This is the first deposit, calculate the initial liquidity
		liquidity = math2.U64GeometryMean(ix.amountA, ix.amountB)
		data := ix.poolInfo.Data().(*poolData)
		if liquidity < data.minLiquidity {
			panic("liquidity less than minimum")
		}
		amountA, amountB = ix.amountA, ix.amountB
	} else if vaultA.Amount != 0 && vaultB.Amount != 0 {
		// Check if amountA and amountB are balanced
		expectedA := math2.U64MulDiv(vaultA.Amount, ix.amountB, vaultB.Amount)
		if expectedA < ix.amountA {
			amountA = expectedA
			amountB = ix.amountB
		} else if expectedA > ix.amountA {
			amountA = ix.amountA
			amountB = math2.U64MulDiv(vaultB.Amount, ix.amountA, vaultA.Amount)
		} else {
			amountA, amountB = ix.amountA, ix.amountB
		}
		liquidity = math2.U64GeometryMean(amountA, amountB)
	} else {
		panic("invalid vault amount, both should be zero or non-zero")
	}

	// Transfer token A/B to the pool
	AbortOnError(token.Transfer(
		ix.tokenA,
		ix.tokenAVault,
		ix.depositor,
		amountA,
		nil,
	))
	AbortOnError(token.Transfer(
		ix.tokenB,
		ix.tokenBVault,
		ix.depositor,
		amountB,
		nil,
	))

	// Mint the liquidity token to the depositor
	mintAuthSeedBump := []SeedBump{{LP_MINT_AUTH_PDA_SEED, ix.mintAuthBump}}
	AbortOnError(token.MintTo(
		ix.mintLiquidity,
		ix.tokenLiquidity,
		ix.mintLpAuth,
		liquidity,
		mintAuthSeedBump,
	))

}
