package solana

func TokenSetAuthority(accountOrMint, currentAuth *AccountInfo, newAuth *PublicKey, authType AuthorityType, signerSeeds []SeedBump) error {
	return solFfi.token_set_authority(accountOrMint.index, currentAuth.index, newAuth, authType, signerSeeds)
}

func TokenTransfer(from, to, auth *AccountInfo, amount uint64, signerSeeds []SeedBump) error {
	return solFfi.token_transfer(from.index, to.index, auth.index, amount, signerSeeds)
}

func TokenCloseAccount(account, dest, auth *AccountInfo, signerSeeds []SeedBump) error {
	return solFfi.token_close_account(account.index, dest.index, auth.index, signerSeeds)
}
