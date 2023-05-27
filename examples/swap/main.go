package main

import (
	"solana"

	_ "./instructions"
)

// This is the entry point of the program
func main() {
	solana.GetIx().Process()
}
