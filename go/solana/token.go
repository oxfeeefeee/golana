package solana

func TokenInitAccount(account, mint, auth, rent *AccountInfo, signerSeeds []SeedBump) error {
	p := solFfi.token_init_account(account.index, mint.index, auth.index, rent.index, signerSeeds)
	return NewSolanaError(p)
}

func TokenCreateAndInitAccount(from, to *AccountInfo, tokenProgram *PublicKey, mint, auth, rent *AccountInfo, signerSeeds []SeedBump) error {
	p := solFfi.token_create_and_init_account(from.index, to.index, tokenProgram, mint.index, auth.index, rent.index, signerSeeds)
	return NewSolanaError(p)
}

func TokenCloseAccount(account, dest, auth *AccountInfo, signerSeeds []SeedBump) error {
	p := solFfi.token_close_account(account.index, dest.index, auth.index, signerSeeds)
	return NewSolanaError(p)
}

func TokenSetAuthority(accountOrMint, currentAuth *AccountInfo, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) error {
	p := solFfi.token_set_authority(accountOrMint.index, currentAuth.index, newAuth, authType, signerSeeds)
	return NewSolanaError(p)
}

func TokenTransfer(from, to, auth *AccountInfo, amount uint64, signerSeeds []SeedBump) error {
	p := solFfi.token_transfer(from.index, to.index, auth.index, amount, signerSeeds)
	return NewSolanaError(p)
}

func TokenCreateAssociatedAccount(from, to, owner, mint, sys, spl *AccountInfo, idempotent bool) error {
	p := solFfi.token_create_associated_account(from.index, to.index, owner.index, mint.index, sys.index, spl.index, idempotent)
	return NewSolanaError(p)
}
