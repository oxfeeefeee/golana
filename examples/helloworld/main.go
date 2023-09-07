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
	// - `account:"mut, signer` for the accounts that are used as signer and writable
	// If you need to access the account data, add the `data:"accountData"` tag to the field
	// where `accountData` is a type name you defined in this package

	// The user's "main" account
	user Account `account:"mut, signer"`
	// The account to be created to store the user's data on chain
	userAccount Account `account:"mut, signer" data:"userData"`

	// Then, list all the programs that are used by the instruction
	// programs are a kind of accounts, but they usually don't get directly referenced in
	// your code. All the programs required by the APIs you use must be listed here.
	// The system program account is used to create the userAccount
	systemProgram Program

	// Finally, list all the instruction parameters
	// Set the initialCount of the greet greater than 0 to cheat
	initialCount uint64
}

type userData struct {
	// Save the Pubkey of the user, so that it won't greet to other users
	auth PublicKey
	// How many times the user has been greeted
	greetCount uint64
}

// This is the business logic of the IxInit
func (ix *IxInit) Process() {
	// On the client side, the userAccount is just a newly generated keypair
	// we now initialize it on chain
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

	names []string
	// This is just to demo array support
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
