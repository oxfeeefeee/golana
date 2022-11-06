package main

import (
	"fmt2"
	"solana"
)

type IxInit struct {
	account1 solana.AccountInfo
	account2 solana.AccountInfo

	account2Data *TestData

	arg1 string
}

type TestData struct {
	i int
	j string
}

func (ix *IxInit) Process() {
	fmt2.Println("IxInit Process")

}

func main() {
	solana.GetIx().Process()
}
