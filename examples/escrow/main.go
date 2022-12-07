package main

import (
	"fmt2"
	. "solana"
)

type IxInit struct {
	//account1 *SignerInfo
	account2 *AccountInfo

	//account1DataMut *TestData
	//account2DataInit  *TestData

	arg1 string
	arg2 int32
}

type TestData struct {
	i int
	j string
}

func (ix *IxInit) Process() error {
	fmt2.Println("IxInit Process--", ix.arg1, ix.arg2)
	ix.arg1 = "qqqqq"
	pk, bump := ix.account2.Key.FindProgramAddress("xxxxx")
	fmt2.Println("IxInit Process", ix.arg1, pk, bump)

	return nil
}

func main() {
	AbortOnError(GetIx().Process())
}
