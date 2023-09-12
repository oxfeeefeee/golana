package solana

import (
	"unsafe"
)

// The address of a Solana Account, it's either coressponding to a private key
// or another account (program) plus a SeedBump
type PublicKey [32]uint8

// The SeedBump is used to identify a PDA (Program Derived Address)
// together with the Program's PublicKey
type SeedBump struct {
	Seed string
	Bump uint8
}

// The Solana Account, represented by a index in the Golana runtime
type Account uint

// The Solana Program is a kind of Account
type Program Account

// Initializes a new Account by calling the solana runtime createAccount function
// payer (account:"mut, signer"): pays for the lamports
// space: the space required for the account
// signerSeeds: used when the singers are PDAs, pass nil if no signer is PDA
// Required Program(s):
//   - SystemProgram
func (account Account) Create(payer Account, space uint64, signerSeeds []SeedBump) error {
	p := solFfi.account_create(payer, account, space, signerSeeds)
	return NewSolanaError(p)
}

// Get Public key of the account
func (account Account) Key() *PublicKey {
	return solFfi.account_key(account)
}

// Get the lamports in the account
func (account Account) Lamports() uint64 {
	return solFfi.account_lamports(account)
}

// Set the lamports in the account
func (account Account) SetLamports(lamports uint64) {
	solFfi.account_set_lamports(account, lamports)
}

// Get the data of the account
func (account Account) Data() interface{} {
	return solFfi.account_data(account)
}

// Save the data of the account
func (account Account) SaveData(data interface{}) {
	solFfi.account_save_data(account, data)
}

// Get the program that owns this account
func (account Account) Owner() *PublicKey {
	return solFfi.account_owner(account)
}

// Get if this account's data contains a loaded program (and is now read-only)
func (account Account) Executable() bool {
	return solFfi.account_executable(account)
}

// Get the epoch at which this account will next owe rent
func (account Account) RentEpoch() uint64 {
	return solFfi.account_rent_epoch(account)
}

// The Instruction interface, can only be returned by the GetIx function
type Ix interface {
	Process()
}

// Returns the current instruction
func GetIx() Ix {
	return solFfi.get_ix()
}

// Returns the current program id, i.e. bytecode PubKey
func GetId() *PublicKey {
	return solFfi.get_id()
}

// Panics if the error is not nil
func AbortOnError(e error) {
	if e != nil {
		panic(e.Error())
	}
}

// Panics if the condition is not true
func Assert(cond bool, msg string) {
	if !cond {
		panic(msg)
	}
}

// Log the remaining compute unit
func LogComputeUnit() {
	solFfi.log_compute_unit()
}

// Find a valid "program derived address"(PDA) and its corresponding bump seed.
func FindProgramAddress(seed string, pk *PublicKey) (*PublicKey, uint8) {
	return solFfi.find_program_address(seed, pk)
}

// For internal use only
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
