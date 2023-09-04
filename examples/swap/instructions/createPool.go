package instructions

import (
	. "solana"
	"token"
)

const VAULT_AUTH_PDA_SEED = "vault-auth"
const LP_MINT_AUTH_PDA_SEED = "mint-auth"

type poolData struct {
	creator      PublicKey
	tokenAVault  PublicKey
	tokenBVault  PublicKey
	minLiquidity uint64
	feeRate      uint64
}

type IxCreatePool struct {
	// The creator of the pool, i.e. the one who called IxCreatePool
	creator Account `account:"mut, signer"`
	// The mint of token A/B
	mintA Account
	mintB Account
	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault Account `account:"mut, signer"`
	tokenBVault Account `account:"mut, signer"`
	// The pool account storing the pool data
	poolInfo      Account `account:"mut" data:"poolData"`
	systemProgram Account
	tokenProgram  Account
	rent          Account

	// The minimum liquidity to deposit, liquidity  = sqrt(amountA * amountB)
	minLiquidity uint64
	// The fee rate, in basis points, i.e. 1000 = 10%
	feeRate uint64
}

func (ix *IxCreatePool) Process() {
	vaultAuthority, _ := FindProgramAddress(VAULT_AUTH_PDA_SEED, GetId())

	// Create the vaults
	AbortOnError(token.CreateAndInitAccount(
		ix.creator,
		ix.tokenAVault,
		ix.mintA,
		vaultAuthority,
		nil))
	AbortOnError(token.CreateAndInitAccount(
		ix.creator,
		ix.tokenBVault,
		ix.mintB,
		vaultAuthority,
		nil))

	// Initialize the pool account
	data := new(poolData)
	data.creator = *ix.creator.Key()
	data.tokenAVault = *ix.tokenAVault.Key()
	data.tokenBVault = *ix.tokenBVault.Key()
	data.minLiquidity = ix.minLiquidity
	data.feeRate = ix.feeRate
	ix.poolInfo.SaveData(data)
}
