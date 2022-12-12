package main

import (
	. "solana"
)

const ESCROW_PDA_SEED = "escrow"

type EscrowAccountData struct {
	initializerKey                 *PublicKey
	initializerDepositTokenAccount *PublicKey
	initializerReceiveTokenAccount *PublicKey
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
	tokenProgram *AccountInfo

	escrowAccount_dataInit *EscrowAccountData

	initializerAmount uint64
	takerAmount       uint64
}

func (ix *IxInit) Process() {
	data := new(EscrowAccountData)
	data.initializerKey = ix.initializer_signer.Key
	data.initializerDepositTokenAccount = ix.initializerDepositTokenAccount.Key
	data.initializerReceiveTokenAccount = ix.initializerReceiveTokenAccount.Key
	data.initializerAmount = ix.initializerAmount
	data.takerAmount = ix.takerAmount
	CommitData(ix.escrowAccount)

	vaultAuthority, _ := FindProgramAddress(ESCROW_PDA_SEED, GetId())
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
}

func (ix *IxExchange) Process() {
	_, bump := FindProgramAddress(ESCROW_PDA_SEED, GetId())
	authority_seeds := []SeedBump{{ESCROW_PDA_SEED, bump}}

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
	vaultAccount,
	vaultAuthority,
	initializerDepositTokenAccount,
	escrowAccount,
	tokenProgram *AccountInfo

	escrowAccount_data *EscrowAccountData
}

func (ix *IxCancel) Process() {
	_, bump := FindProgramAddress(ESCROW_PDA_SEED, GetId())
	authority_seeds := []SeedBump{{ESCROW_PDA_SEED, bump}}

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
