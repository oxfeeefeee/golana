package solana

type PublicKey [32]uint8

type AccountInfo struct {
	/// Public key of the account
	Key PublicKey
	/// Was the transaction signed by this account's public key?
	IsSigner bool
	/// Is the account writable?
	IsWritable bool
	/// The lamports in the account.  Modifiable by programs.
	Lamports *uint64
	/// Program that owns this account
	Owner PublicKey
	/// This account's data contains a loaded program (and is now read-only)
	Executable bool
	/// The epoch at which this account will next owe rent
	RentEpoch uint64
}

var nativeSolana ffiSolana

func init() {
	nativeSolana = ffi(ffiSolana, "solana")
}

type ffiSolana interface {
	/// Get current solana Instruction
	get_ix() Ix
}

type Ix interface {
	Process()
}

func GetIx() Ix {
	return nativeSolana.get_ix()
}