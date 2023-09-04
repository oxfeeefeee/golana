package solana

import (
	"unsafe"
)

var solFfi ffiSolana

func init() {
	solFfi = ffi(ffiSolana, "solana")
}

type ffiSolana interface {
	/// Get current solana Instruction
	get_ix() Ix

	get_id() *PublicKey

	account_create(from, account Account, space uint64, signerSeeds []SeedBump) unsafe.Pointer

	account_key(account Account) *PublicKey

	account_lamports(account Account) uint64

	account_set_lamports(account Account, lamports uint64)

	account_owner(account Account) *PublicKey

	account_executable(account Account) bool

	account_rent_epoch(account Account) uint64

	account_data(account Account) interface{}

	account_save_data(account Account, data interface{})

	error_string(ptr unsafe.Pointer) string

	log_compute_unit()

	find_program_address(seed string, program *PublicKey) (*PublicKey, uint8)
}
