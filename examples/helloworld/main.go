package main

import (
	"fmt2"
	. "solana"
)

// This is the definition of the IxInit Instruction
type IxInit struct {
	// First, list all the accounts that are used by the instruction
	// Use tags to specify the account attributes:
	// - `account:"signer"` for the accounts that are used as signer
	// - `account:"mut"` for the accounts that are used as writable
	user          Account `account:"mut, signer"`
	userAccount   Account `account:"mut, signer" data:"userData"`
	systemProgram Account

	// Second, declare the data stored in the accounts, that needs to be read or written by the instruction
	// Use the corresponding account name with a _data suffix,
	// and add the `account_data:"readonly"`,  `account_data:"init"` or `account_data:"mut"` tag to the field:
	// - `account_data:"readonly"` for the data that will be readonly
	// - `account_data:"init"` for the data that will be initialized by the instruction
	// - `account_data:"mut"` for the data that will be written by the instruction
	//userAccount_data *userData `account_data:"init"`

	// Finally, list all the instruction parameters
	initialCount uint64
}

type userData struct {
	auth       PublicKey
	greetCount uint64
}

// This is the business logic of the IxInit
func (ix *IxInit) Process() {
	ix.userAccount.Create(ix.user, 512, nil)

	data := new(userData)
	// set the auth of userAccount as the user
	data.auth = *ix.user.Key()
	data.greetCount = ix.initialCount
	ix.userAccount.SaveData(data)
}

type IxGreet struct {
	user        Account `account:"signer"`
	userAccount Account `account:"mut" data:"userData"`

	//userAccount_data *userData `account_data:"mut"`

	names     []string
	arrayTest [3]int64
}

func (ix *IxGreet) Process() {
	data := ix.userAccount.Data().(*userData)
	// Check that the user is the auth of the userAccount
	assert(*ix.user.Key() == data.auth)

	fmt2.Println("Hello", ix.names, "you have been greeted", data.greetCount, "times", "arrayTest", ix.arrayTest)

	// Increment the greetCount
	data.greetCount++
	ix.userAccount.SaveData(data)

}

// This is the entry point of the program
func main() {
	GetIx().Process()
}
