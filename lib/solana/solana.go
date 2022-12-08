package solana

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
	index uint
}

func (pk *PublicKey) FindProgramAddress(seed string) (*PublicKey, uint8) {
	return solFfi.find_program_address(seed, pk)
}

type SignerInfo AccountInfo

type Ix interface {
	Process() error
}

func GetIx() Ix {
	return solFfi.get_ix()
}

func CommitLamports(account *AccountInfo) {
	solFfi.commit_lamports(account.index)
}

func CommitData(account *AccountInfo) {
	solFfi.commit_data(account.index)
}

func CommitLamportsAndData(account *AccountInfo) {
	solFfi.commit_lamports_and_data(account.index)
}

func CommitEverything() {
	solFfi.commit_everything()
}

func AbortOnError(e error) {
	if e != nil {
		panic(e)
	}
}
