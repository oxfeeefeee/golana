package solana

import "unsafe"

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

	find_program_address(seed string, program *PublicKey) (*PublicKey, uint8)

	create_account(from, to uint, owner *PublicKey, lamports, space uint64, signerSeeds []SeedBump) unsafe.Pointer

	token_init_account(account, mint, auth, rent uint, signerSeeds []SeedBump) unsafe.Pointer

	token_create_and_init_account(from, to uint, tokenProgram *PublicKey, mint, auth, rent uint, signerSeeds []SeedBump) unsafe.Pointer

	token_close_account(account, dest, auth uint, signerSeeds []SeedBump) unsafe.Pointer

	token_set_authority(accountOrMint, currentAuth uint, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) unsafe.Pointer

	token_transfer(from, to, auth uint, amount uint64, signerSeeds []SeedBump) unsafe.Pointer
}

type SolanaError struct {
	ptr unsafe.Pointer
}

func NewSolanaError(ptr unsafe.Pointer) *SolanaError {
	if ptr == nil {
		return nil
	}
	return &SolanaError{ptr}
}

func (e *SolanaError) Error() string {
	return solFfi.error_string(e.ptr)
}
