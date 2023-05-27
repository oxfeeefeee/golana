package instructions

import (
	"fmt2"
	_ "solana"
)

// This is the definition of the IxInit Instruction
type IxInit struct {
	// First, list all the accounts that are used by the instruction
	// Use tags to specify the account attributes:
	// - `golana:"signer"` for the accounts that are used as signer
	// - `golana:"mut"` for the accounts that are used as writable
	// ...

	// Second, declare the data stored in the accounts, that needs to be read or written by the instruction
	// Use the corresponding account name with a _data suffix,
	// and add the `golana:"init"` or `golana:"mut"` tag to the field:
	// - `golana:"init"` for the data that will be initialized by the instruction
	// - `golana:"mut"` for the data that will be written by the instruction
	// ...

	// Finally, list all the instruction parameters
	// ...
}

// This is the business logic of the IxInit
func (ix *IxInit) Process() {
	// ...
}

func SomeFunc() {
	fmt2.Println("someFunc from somelib")
}
