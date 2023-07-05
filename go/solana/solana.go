package solana

import (
	"unsafe"
)

type PublicKey [32]uint8

type SeedBump struct {
	Seed string
	Bump uint8
}

type AuthorityType uint8

const (
	// Authority to mint new tokens
	AuthMintTokens AuthorityType = iota
	// Authority to freeze any account associated with the Mint
	AuthFreezeAccount
	// Owner of a given token account
	AuthAccountOwner
	// Authority to close a token account
	AuthCloseAccount
)

type AccountInfo struct {
	// Public key of the account
	Key *PublicKey
	// The lamports in the account.  Modifiable by programs.
	Lamports uint64
	// Program that owns this account
	Owner *PublicKey
	// This account's data contains a loaded program (and is now read-only)
	Executable bool
	// The epoch at which this account will next owe rent
	RentEpoch uint64
	// For internal use, do not access
	Index uint
}

type Ix interface {
	Process()
}

func GetIx() Ix {
	return solFfi.get_ix()
}

func GetId() *PublicKey {
	return solFfi.get_id()
}

func CommitLamports(account *AccountInfo) {
	solFfi.commit_lamports(account.Index)
}

func CommitData(account *AccountInfo) {
	solFfi.commit_data(account.Index)
}

func CommitLamportsAndData(account *AccountInfo) {
	solFfi.commit_lamports_and_data(account.Index)
}

func CommitEverything() {
	solFfi.commit_everything()
}

func AbortOnError(e error) {
	if e != nil {
		panic(e)
	}
}

func LogComputeUnit() {
	solFfi.log_compute_unit()
}

func FindProgramAddress(seed string, pk *PublicKey) (*PublicKey, uint8) {
	return solFfi.find_program_address(seed, pk)
}

func CreateAccount(from, to *AccountInfo, owner *PublicKey, lamports, space uint64, signerSeeds []SeedBump) error {
	p := solFfi.create_account(from.Index, to.Index, owner, lamports, space, signerSeeds)
	return NewSolanaError(p)
}

type SolanaError struct {
	ptr unsafe.Pointer
}

func NewSolanaError(ptr unsafe.Pointer) error {
	if ptr == nil {
		return nil
	}
	return &SolanaError{ptr}
}

func (e *SolanaError) Error() string {
	return solFfi.error_string(e.ptr)
}
