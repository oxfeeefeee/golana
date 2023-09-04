package solana

import (
	"unsafe"
)

type PublicKey [32]uint8

type SeedBump struct {
	Seed string
	Bump uint8
}

type Account uint

// Initializes a new Account by calling the solana runtime createAccount function
func (account Account) Create(from Account, owner *PublicKey, lamports, space uint64, signerSeeds []SeedBump) error {
	p := solFfi.account_create(from, account, owner, lamports, space, signerSeeds)
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

type Ix interface {
	Process()
}

func GetIx() Ix {
	return solFfi.get_ix()
}

func GetId() *PublicKey {
	return solFfi.get_id()
}

func AbortOnError(e error) {
	if e != nil {
		panic(e.Error())
	}
}

func LogComputeUnit() {
	solFfi.log_compute_unit()
}

func FindProgramAddress(seed string, pk *PublicKey) (*PublicKey, uint8) {
	return solFfi.find_program_address(seed, pk)
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
