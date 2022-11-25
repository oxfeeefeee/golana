package main

import (
	"fmt2"
	"solana"
)

type IxInit struct {
	//account1 solana.SignerInfo
	account2 solana.AccountInfo

	//account1DataMut *TestData
	//account2DataInit  *TestData

	arg1 string
	arg2 int32
}

type TestData struct {
	i int
	j string
}

func (ix *IxInit) Process() {
	fmt2.Println("IxInit Process", ix.arg1, ix.arg2)
	ix.arg1 = "qqqqq"
	fmt2.Println("IxInit Process", ix.arg1)

}

func main() {
	solana.GetIx().Process()
}
