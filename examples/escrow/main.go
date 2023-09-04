// This is ported from https://github.com/ironaddicteddog/anchor-escrow

package main

import (
	. "solana"
	"token"
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
	// Use tags to specify the account attributes:
	// - `golana:"signer"` for the accounts that are used as signer
	// - `golana:"mut"` for the accounts that are used as writable
	initializer                    Account `account:"mut, signer"`
	mint                           Account
	vaultAccount                   Account `account:"mut"`
	initializerDepositTokenAccount Account `account:"mut"`
	initializerReceiveTokenAccount Account
	escrowAccount                  Account `account:"mut" data:"EscrowAccountData"`
	systemProgram                  Account
	rent                           Account
	tokenProgram                   Account

	// Second, declare the data stored in the accounts, that needs to be read or written by the instruction
	// Use the corresponding account name with a _data suffix,
	// and add the `golana:"init"` or `golana:"mut"` tag to the field:
	// - `golana:"init"` for the data that will be initialized by the instruction
	// - `golana:"mut"` for the data that will be written by the instruction
	// escrowAccount_data *EscrowAccountData `golana:"init"`

	// Finally, list all the instruction parameters
	vaultAccountBump  uint8
	initializerAmount uint64
	takerAmount       uint64
}

// This is the business logic of the Init instruction
func (ix *IxInit) Process() {
	// First, stores the data in the escrow account
	data := new(EscrowAccountData)
	data.initializerKey = *ix.initializer.Key()
	data.initializerDepositTokenAccount = *ix.initializerDepositTokenAccount.Key()
	data.initializerReceiveTokenAccount = *ix.initializerReceiveTokenAccount.Key()
	data.initializerAmount = ix.initializerAmount
	data.takerAmount = ix.takerAmount
	// Then, commit the data to the account
	ix.escrowAccount.SaveData(data)

	vault_seeds := []SeedBump{{VAULT_PDA_SEED, ix.vaultAccountBump}}
	vaultAuthority, _ := FindProgramAddress(ESCROW_PDA_SEED, GetId())
	AbortOnError(token.CreateAndInitAccount(
		ix.initializer,
		ix.vaultAccount,
		ix.mint,
		vaultAuthority,
		vault_seeds))
	AbortOnError(token.Transfer(
		ix.initializerDepositTokenAccount,
		ix.vaultAccount,
		ix.initializer,
		ix.initializerAmount, nil))
}

type IxExchange struct {
	taker                          Account `account:"signer"`
	takerDepositTokenAccount       Account `account:"mut"`
	takerReceiveTokenAccount       Account `account:"mut"`
	initializer                    Account `account:"mut"`
	initializerDepositTokenAccount Account `account:"mut"`
	initializerReceiveTokenAccount Account `account:"mut"`
	escrowAccount                  Account `account:"mut" data:"EscrowAccountData"`
	vaultAccount                   Account `account:"mut"`
	vaultAuthority                 Account
	tokenProgram                   Account

	escrowBump uint8
}

func (ix *IxExchange) Process() {
	// assert is a built-in function added by the Goscript compiler
	data := ix.escrowAccount.Data().(*EscrowAccountData)
	assert(*ix.initializer.Key() == data.initializerKey)
	assert(*ix.initializerDepositTokenAccount.Key() == data.initializerDepositTokenAccount)
	assert(*ix.initializerReceiveTokenAccount.Key() == data.initializerReceiveTokenAccount)

	authority_seeds := []SeedBump{{ESCROW_PDA_SEED, ix.escrowBump}}
	AbortOnError(token.Transfer(
		ix.takerDepositTokenAccount,
		ix.initializerReceiveTokenAccount,
		ix.taker,
		data.takerAmount, nil))
	AbortOnError(token.Transfer(
		ix.vaultAccount,
		ix.takerReceiveTokenAccount,
		ix.vaultAuthority,
		data.initializerAmount,
		authority_seeds))
	AbortOnError(token.CloseAccount(
		ix.vaultAccount,
		ix.initializer,
		ix.vaultAuthority,
		authority_seeds))

}

type IxCancel struct {
	initializer                    Account `account:"signer, mut"`
	initializerDepositTokenAccount Account `account:"mut"`
	vaultAccount                   Account `account:"mut"`
	vaultAuthority                 Account
	escrowAccount                  Account `account:"mut" data:"EscrowAccountData"`
	tokenProgram                   Account

	escrowBump uint8
}

func (ix *IxCancel) Process() {
	data := ix.escrowAccount.Data().(*EscrowAccountData)
	assert(*ix.initializer.Key() == data.initializerKey)
	assert(*ix.initializerDepositTokenAccount.Key() == data.initializerDepositTokenAccount)

	//_, bump := FindProgramAddress(ESCROW_PDA_SEED, GetId())
	authority_seeds := []SeedBump{{ESCROW_PDA_SEED, ix.escrowBump}}

	AbortOnError(token.Transfer(
		ix.vaultAccount,
		ix.initializerDepositTokenAccount,
		ix.vaultAuthority,
		data.initializerAmount,
		authority_seeds))
	AbortOnError(token.CloseAccount(
		ix.vaultAccount,
		ix.initializer,
		ix.vaultAuthority,
		authority_seeds))

}

// This is the entry point of the program
func main() {
	GetIx().Process()
}
