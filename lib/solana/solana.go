package solana

type PublicKey [32]uint8

type SeedBump struct {
	Seed string
	Bump uint8
}

type AccountInfo struct {
	/// Public key of the account
	Key *PublicKey
	/// The lamports in the account.  Modifiable by programs.
	Lamports *uint64
	/// Program that owns this account
	Owner *PublicKey
	/// This account's data contains a loaded program (and is now read-only)
	Executable bool
	/// The epoch at which this account will next owe rent
	RentEpoch uint64
}

type SignerInfo AccountInfo

type Ix interface {
	Process()
}

func GetIx() Ix {
	return solFfi.get_ix()
}
