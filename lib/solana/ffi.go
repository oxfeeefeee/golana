package solana

var solFfi ffiSolana

func init() {
	solFfi = ffi(ffiSolana, "solana")
}

type ffiSolana interface {
	/// Get current solana Instruction
	get_ix() Ix

	commit_lamports(index uint)

	commit_data(index uint)

	commit_all(index uint)

	abort_on_error(e error)

	find_program_address(seed string, program *PublicKey) (*PublicKey, uint8)

	token_set_authority(accountOrMint, currentAuth uint, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) error

	token_transfer(from, to, auth uint, amount uint64, signerSeeds []SeedBump) error

	token_close_account(account, dest, auth uint, signerSeeds []SeedBump) error
}
