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

	commit_lamports(index uint)

	commit_data(index uint)

	commit_lamports_and_data(index uint)

	commit_everything()

	error_string(ptr unsafe.Pointer) string

	log_compute_unit()

	find_program_address(seed string, program *PublicKey) (*PublicKey, uint8)

	create_account(from, to uint, owner *PublicKey, lamports, space uint64, signerSeeds []SeedBump) unsafe.Pointer
}
