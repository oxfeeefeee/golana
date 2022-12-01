package solana

func TokenSetAuthority(accountOrMint, currentAuth, newAuth *PublicKey) error {
	return solFfi.token_set_authority(accountOrMint, currentAuth, newAuth)
}

func TokenTransfer(from, to, auth *PublicKey, amount uint64, signerSeeds []SeedBump) error {
	return solFfi.token_transfer(from, to, auth, amount, signerSeeds)
}

func TokenCloseAccount(account, dest, auth *PublicKey, signerSeeds []SeedBump) error {
	return solFfi.token_close_account(account, dest, auth, signerSeeds)
}
