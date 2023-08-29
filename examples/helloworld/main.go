package main

import (
	"fmt2"
	. "solana"
)

// This is the definition of the IxInit Instruction
type IxInit struct {
	// First, list all the accounts that are used by the instruction
	// Use tags to specify the account attributes:
	// - `golana:"signer"` for the accounts that are used as signer
	// - `golana:"mut"` for the accounts that are used as writable
	user        *AccountInfo `golana:"signer"`
	userAccount *AccountInfo `golana:"mut"`

	// Second, declare the data stored in the accounts, that needs to be read or written by the instruction
	// Use the corresponding account name with a _data suffix,
	// and add the `golana:"init"` or `golana:"mut"` tag to the field:
	// - `golana:"init"` for the data that will be initialized by the instruction
	// - `golana:"mut"` for the data that will be written by the instruction
	userAccount_data *UserData `golana:"init"`

	// Finally, list all the instruction parameters
	initialCount uint64
}

type UserData struct {
	auth       PublicKey
	greetCount uint64
}

// This is the business logic of the IxInit
func (ix *IxInit) Process() {
	data := new(UserData)
	// set the auth of userAccount as the user
	data.auth = *ix.user.Key
	data.greetCount = ix.initialCount
	ix.userAccount_data = data
	CommitData(ix.userAccount)
}

type IxGreet struct {
	user        *AccountInfo `golana:"signer"`
	userAccount *AccountInfo `golana:"mut"`

	userAccount_data *UserData `golana:"mut"`

	name string
}

func (ix *IxGreet) Process() {
	// Check that the user is the auth of the userAccount
	assert(*ix.user.Key == ix.userAccount_data.auth)

	fmt2.Println("Hello", ix.name, "you have been greeted", ix.userAccount_data.greetCount, "times")

	// Increment the greetCount
	ix.userAccount_data.greetCount++
	CommitData(ix.userAccount)

	//AbortOnError(errors.New("AbortOnError test"))

}

// This is the entry point of the program
func main() {
	GetIx().Process()
}
