package main

import (
	. "solana"
)

const ESCROW_PDA_SEED = "escrow"
const VAULT_PDA_SEED = "token-seed"

type EscrowAccountData struct {
	initializerKey                 PublicKey
	initializerDepositTokenAccount PublicKey
	initializerReceiveTokenAccount PublicKey
	initializerAmount              uint64
	takerAmount                    uint64
}

type IxInit struct {
	initializer_signer,
	mint,
	vaultAccount,
	initializerDepositTokenAccount,
	initializerReceiveTokenAccount,
	escrowAccount,
	systemProgram,
	rent,
	tokenProgram *AccountInfo

	escrowAccount_dataInit *EscrowAccountData

	vaultAccountBump  uint8
	initializerAmount uint64
	takerAmount       uint64
}

func (ix *IxInit) Process() {
	data := new(EscrowAccountData)
	data.initializerKey = *ix.initializer_signer.Key
	data.initializerDepositTokenAccount = *ix.initializerDepositTokenAccount.Key
	data.initializerReceiveTokenAccount = *ix.initializerReceiveTokenAccount.Key
	data.initializerAmount = ix.initializerAmount
	data.takerAmount = ix.takerAmount
	ix.escrowAccount_dataInit = data
	CommitData(ix.escrowAccount)

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

func main() {
	GetIx().Process()
}
