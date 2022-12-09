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
	initializer                    *SignerInfo
	mint                           *AccountInfo
	vaultAccount                   *AccountInfo
	initializerDepositTokenAccount *AccountInfo
	initializerReceiveTokenAccount *AccountInfo
	escrowAccount                  *AccountInfo
	tokenProgram                   *AccountInfo

	escrowAccountDataInit *EscrowAccountData

	initializerAmount uint64
	takerAmount       uint64
}

func (ix *IxInit) Process() {
	data := new(EscrowAccountData)
	data.initializerKey = ix.initializer.Key
	data.initializerDepositTokenAccount = ix.initializerDepositTokenAccount.Key
	data.initializerReceiveTokenAccount = ix.initializerReceiveTokenAccount.Key
	data.initializerAmount = ix.initializerAmount
	data.takerAmount = ix.takerAmount
	CommitData(ix.escrowAccount)

	vaultAuthority, _ := GetId().FindProgramAddress(ESCROW_PDA_SEED)
	AbortOnError(TokenSetAuthority(
		ix.vaultAccount,
		(*AccountInfo)(ix.initializer),
		vaultAuthority,
		AuthAccountOwner, nil))
	AbortOnError(TokenTransfer(
		ix.initializerDepositTokenAccount,
		ix.vaultAccount,
		(*AccountInfo)(ix.initializer),
		ix.initializerAmount, nil))
}

type IxExchange struct {
	taker                          *SignerInfo
	takerDepositTokenAccount       *AccountInfo
	takerReceiveTokenAccount       *AccountInfo
	initializer                    *AccountInfo
	initializerDepositTokenAccount *AccountInfo
	initializerReceiveTokenAccount *AccountInfo
	escrowAccount                  *AccountInfo
	vaultAccount                   *AccountInfo
	vaultAuthority                 *AccountInfo
	tokenProgram                   *AccountInfo

	escrowAccountData *EscrowAccountData
}

func (ix *IxExchange) Process() {
	_, bump := GetId().FindProgramAddress(ESCROW_PDA_SEED)
	authority_seeds := []SeedBump{{ESCROW_PDA_SEED, bump}}

	AbortOnError(TokenTransfer(
		ix.takerDepositTokenAccount,
		ix.initializerReceiveTokenAccount,
		(*AccountInfo)(ix.taker),
		ix.escrowAccountData.takerAmount, nil))
	AbortOnError(TokenTransfer(
		ix.vaultAccount,
		ix.takerReceiveTokenAccount,
		ix.vaultAuthority,
		ix.escrowAccountData.initializerAmount,
		authority_seeds))
	AbortOnError(TokenCloseAccount(
		ix.vaultAccount,
		ix.initializer,
		ix.vaultAuthority,
		authority_seeds))

}

type IxCancel struct {
	initializer                    *SignerInfo
	vaultAccount                   *AccountInfo
	vaultAuthority                 *AccountInfo
	initializerDepositTokenAccount *AccountInfo
	escrowAccount                  *AccountInfo
	tokenProgram                   *AccountInfo

	escrowAccountData *EscrowAccountData
}

func (ix *IxCancel) Process() {
	_, bump := GetId().FindProgramAddress(ESCROW_PDA_SEED)
	authority_seeds := []SeedBump{{ESCROW_PDA_SEED, bump}}

	AbortOnError(TokenTransfer(
		ix.vaultAccount,
		ix.initializerDepositTokenAccount,
		ix.vaultAuthority,
		ix.escrowAccountData.initializerAmount,
		authority_seeds))
	AbortOnError(TokenCloseAccount(
		ix.vaultAccount,
		(*AccountInfo)(ix.initializer),
		ix.vaultAuthority,
		authority_seeds))

}

func main() {
	GetIx().Process()
}
