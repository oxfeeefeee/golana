package instructions

import (
	. "solana"
	"token"
)

const AUTH_PDA_SEED = "auth"
const TOKEN_A_VAULT_SEED = "token-a"
const TOKEN_B_VAULT_SEED = "token-b"

type poolData struct {
	creator     PublicKey
	tokenAVault PublicKey
	tokenBVault PublicKey
	feeRate     uint64
}

type IxCreatePool struct {
	// The creator of the pool, i.e. the one who called IxCreatePool
	creator *AccountInfo `golana:"mut, signer"`
	// The mint of token A/B
	mintA *AccountInfo
	mintB *AccountInfo
	// The vault holding token A/B, i.e. the SPL token account
	tokenAVault *AccountInfo `golana:"mut"`
	tokenBVault *AccountInfo `golana:"mut"`
	// The pool account storing the pool data
	poolInfo      *AccountInfo `golana:"mut"`
	systemProgram *AccountInfo
	tokenProgram  *AccountInfo
	rent          *AccountInfo

	poolInfo_data *poolData `golana:"init"`

	// The fee rate, in basis points, i.e. 1000 = 10%
	feeRate         uint64
	tokenAVaultBump uint8
	tokenBVaultBump uint8
}

func (ix *IxCreatePool) Process() {
	vaultASeedBump := []SeedBump{{TOKEN_A_VAULT_SEED, ix.tokenAVaultBump}}
	vaultBSeedBump := []SeedBump{{TOKEN_B_VAULT_SEED, ix.tokenBVaultBump}}
	vaultAuthority, _ := FindProgramAddress(AUTH_PDA_SEED, GetId())

	// Create the vaults
	AbortOnError(token.CreateAndInitAccount(
		ix.creator,
		ix.tokenAVault,
		ix.tokenProgram.Key,
		ix.mintA,
		ix.creator,
		ix.rent,
		vaultASeedBump))
	AbortOnError(token.SetAuthority(
		ix.tokenAVault,
		ix.creator,
		vaultAuthority,
		AuthAccountOwner, nil))
	AbortOnError(token.CreateAndInitAccount(
		ix.creator,
		ix.tokenBVault,
		ix.tokenProgram.Key,
		ix.mintB,
		ix.creator,
		ix.rent,
		vaultBSeedBump))
	AbortOnError(token.SetAuthority(
		ix.tokenBVault,
		ix.creator,
		vaultAuthority,
		AuthAccountOwner, nil))

	// Initialize the pool account
	data := new(poolData)
	data.creator = *ix.creator.Key
	data.tokenAVault = *ix.tokenAVault.Key
	data.tokenBVault = *ix.tokenBVault.Key
	data.feeRate = ix.feeRate
	ix.poolInfo_data = data
	// Commit the data to the account
	CommitData(ix.poolInfo)
}
