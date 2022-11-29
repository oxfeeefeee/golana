package solana

var solFfi ffiSolana

func init() {
	solFfi = ffi(ffiSolana, "solana")
}

type ffiSolana interface {
	/// Get current solana Instruction
	get_ix() Ix

	find_program_address(program *PublicKey) (*PublicKey, uint8)

	token_set_authority(accountOrMint, currentAuth, newAuth *PublicKey) error

	token_transfer(from, to, auth *PublicKey, amount uint64, signerSeeds []SeedBump) error

	token_close_account(account, dest, auth *PublicKey, signerSeeds []SeedBump) error
}
