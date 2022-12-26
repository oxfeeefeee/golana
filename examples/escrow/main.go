// This is ported from https://github.com/ironaddicteddog/anchor-escrow

package main

import (
	. "solana"
)

const ESCROW_PDA_SEED = "escrow"
const VAULT_PDA_SEED = "token-seed"

// The information related to the escrow transaction stored in the escrow Account
type EscrowAccountData struct {
	initializerKey                 PublicKey
	initializerDepositTokenAccount PublicKey
	initializerReceiveTokenAccount PublicKey
	initializerAmount              uint64
	takerAmount                    uint64
}

// This is the definition of the Init Instruction
type IxInit struct {
	// First, list all the accounts that are used by the instruction
	initializer_signer, // Use _signer suffix for accounts that are used as signer
	mint,
	vaultAccount,
	initializerDepositTokenAccount,
	initializerReceiveTokenAccount,
	escrowAccount,
	systemProgram,
	rent,
	tokenProgram *AccountInfo

	// Second, declare the data stored in the accounts, that needs to be read or written by the instruction
	// Use the account name as prefix, and _dataXXXX suffix:
	// - dataInit for the data that is initialized by the instruction
	// - data for the data that is read by the instruction
	// - dataMut for the data that is written by the instruction
	escrowAccount_dataInit *EscrowAccountData

	// Finally, list all the instruction parameters
	vaultAccountBump  uint8
	initializerAmount uint64
	takerAmount       uint64
}

// This is the business logic of the Init instruction
func (ix *IxInit) Process() {
	// First, stores the data in the escrow account
	data := new(EscrowAccountData)
	data.initializerKey = *ix.initializer_signer.Key
	data.initializerDepositTokenAccount = *ix.initializerDepositTokenAccount.Key
	data.initializerReceiveTokenAccount = *ix.initializerReceiveTokenAccount.Key
	data.initializerAmount = ix.initializerAmount
	data.takerAmount = ix.takerAmount
	ix.escrowAccount_dataInit = data
	// Then, commit the data to the account
	CommitData(ix.escrowAccount)

	// The following code is pretty much the same as the original anchor version
	vault_seeds := []SeedBump{{VAULT_PDA_SEED, ix.vaultAccountBump}}
	vaultAuthority, _ := FindProgramAddress(ESCROW_PDA_SEED, GetId())
	AbortOnError(TokenCreateAndInitAccount(
		ix.initializer_signer,
		ix.vaultAccount,
		ix.tokenProgram.Key,
		ix.mint,
		ix.initializer_signer,
		ix.rent,
		vault_seeds))
	AbortOnError(TokenSetAuthority(
		ix.vaultAccount,
		ix.initializer_signer,
		vaultAuthority,
		AuthAccountOwner, nil))
	AbortOnError(TokenTransfer(
		ix.initializerDepositTokenAccount,
		ix.vaultAccount,
		ix.initializer_signer,
		ix.initializerAmount, nil))
}

type IxExchange struct {
	taker_signer,
	takerDepositTokenAccount,
	takerReceiveTokenAccount,
	initializer,
	initializerDepositTokenAccount,
	initializerReceiveTokenAccount,
	escrowAccount,
	vaultAccount,
	vaultAuthority,
	tokenProgram *AccountInfo

	escrowAccount_data *EscrowAccountData

	escrowBump uint8
}

func (ix *IxExchange) Process() {
	assert(*ix.initializer.Key == ix.escrowAccount_data.initializerKey)
	assert(*ix.initializerDepositTokenAccount.Key == ix.escrowAccount_data.initializerDepositTokenAccount)
	assert(*ix.initializerReceiveTokenAccount.Key == ix.escrowAccount_data.initializerReceiveTokenAccount)

	authority_seeds := []SeedBump{{ESCROW_PDA_SEED, ix.escrowBump}}
	AbortOnError(TokenTransfer(
		ix.takerDepositTokenAccount,
		ix.initializerReceiveTokenAccount,
		ix.taker_signer,
		ix.escrowAccount_data.takerAmount, nil))
	AbortOnError(TokenTransfer(
		ix.vaultAccount,
		ix.takerReceiveTokenAccount,
		ix.vaultAuthority,
		ix.escrowAccount_data.initializerAmount,
		authority_seeds))
	AbortOnError(TokenCloseAccount(
		ix.vaultAccount,
		ix.initializer,
		ix.vaultAuthority,
		authority_seeds))

}

type IxCancel struct {
	initializer_signer,
	initializerDepositTokenAccount,
	vaultAccount,
	vaultAuthority,
	escrowAccount,
	tokenProgram *AccountInfo

	escrowAccount_data *EscrowAccountData

	escrowBump uint8
}

func (ix *IxCancel) Process() {
	assert(*ix.initializer_signer.Key == ix.escrowAccount_data.initializerKey)
	assert(*ix.initializerDepositTokenAccount.Key == ix.escrowAccount_data.initializerDepositTokenAccount)

	//_, bump := FindProgramAddress(ESCROW_PDA_SEED, GetId())
	authority_seeds := []SeedBump{{ESCROW_PDA_SEED, ix.escrowBump}}

	AbortOnError(TokenTransfer(
		ix.vaultAccount,
		ix.initializerDepositTokenAccount,
		ix.vaultAuthority,
		ix.escrowAccount_data.initializerAmount,
		authority_seeds))
	AbortOnError(TokenCloseAccount(
		ix.vaultAccount,
		ix.initializer_signer,
		ix.vaultAuthority,
		authority_seeds))

}

// This is the entry point of the program
func main() {
	GetIx().Process()
}
