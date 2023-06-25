package instructions

import (
	. "solana"
)

type IxClosePool struct {
	// The creator of the pool, i.e. the one who called IxClosePool
	creator *AccountInfo `golana:"mut, signer"`
	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault *AccountInfo
	tokenBVault *AccountInfo
	// The pool account storing the pool data
	poolInfo      *AccountInfo `golana:"mut"`
	systemProgram *AccountInfo
	tokenProgram  *AccountInfo

	poolInfo_data *poolData `golana:"init"`

	tokenAVaultBump uint8
	tokenBVaultBump uint8
}

func (ix *IxClosePool) Process() {
	// vaultASeedBump := []SeedBump{{TOKEN_A_VAULT_SEED, ix.tokenAVaultBump}}
	// vaultBSeedBump := []SeedBump{{TOKEN_B_VAULT_SEED, ix.tokenBVaultBump}}
	// vaultAuthority, _ := FindProgramAddress(AUTH_PDA_SEED, GetId())

	// // Create the vaults
	// AbortOnError(TokenCreateAndInitAccount(
	// 	ix.creator,
	// 	ix.tokenAVault,
	// 	ix.tokenProgram.Key,
	// 	ix.mintA,
	// 	vaultAuthority,
	// 	ix.rent,
	// 	vaultASeedBump))
	// AbortOnError(TokenCreateAndInitAccount(
	// 	ix.creator,
	// 	ix.tokenBVault,
	// 	ix.tokenProgram.Key,
	// 	ix.mintB,
	// 	vaultAuthority,
	// 	ix.rent,
	// 	vaultBSeedBump))

	// // Initialize the pool account
	// data := new(poolData)
	// data.creator = *ix.creator.Key
	// data.tokenAVault = *ix.tokenAVault.Key
	// data.tokenBVault = *ix.tokenBVault.Key
	// data.feeRate = ix.feeRate
	// ix.poolInfo_data = data
	// // Commit the data to the account
	// CommitData(ix.poolInfo)
}
