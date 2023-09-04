import { IDL, Swap } from '../target/swap_idl.js';
import { Program, initFromEnv } from "golana";
import { ComputeBudgetProgram, Keypair, SystemProgram, Transaction, PublicKey, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, mintTo, getAccount, getOrCreateAssociatedTokenAccount, Account, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import BN from 'bn.js';
import { assert } from "chai";

describe("swap", async () => {
    try {
        const provider = initFromEnv();

        const swap =  await Program.create<Swap>(IDL, provider.publicKey);

        const infoAccountSpace = 512;

        const infoAccount = Keypair.generate();
        const mintAuthority = Keypair.generate();
        const vaultA = Keypair.generate();
        const vaultB = Keypair.generate();

        const creator = Keypair.generate();
        const depositor = Keypair.generate();
        const trader = Keypair.generate();

        let depositorTokenAccountA: Account;
        let depositorTokenAccountB: Account;
        let depositorTokenAccountLP: Account;

        let traderTokenAccountA: Account;
        let traderTokenAccountB: Account;

        let mintA: PublicKey;
        let mintB: PublicKey;
        let mintLP: PublicKey;

        let vault_authority_pda: PublicKey;
        let vault_authority_bump: number;
        let lp_token_mint_auth_pda: PublicKey;
        let lp_token_mint_auth_bump: number;



        it("Initialize program state", async () => {
            ([vault_authority_pda, vault_authority_bump] = await swap.findAddr("vault-auth"));
            ([lp_token_mint_auth_pda, lp_token_mint_auth_bump] = await swap.findAddr("mint-auth"));

            // Airdropping tokens
            await provider.connection.confirmTransaction(
                await provider.connection.requestAirdrop(creator.publicKey, 1000000000),
                "processed"
            );
            await provider.connection.confirmTransaction(
                await provider.connection.requestAirdrop(depositor.publicKey, 1000000000),
                "processed"
            );
            await provider.connection.confirmTransaction(
                await provider.connection.requestAirdrop(trader.publicKey, 1000000000),
                "processed"
            );

            const infoAccountLamports = await provider.connection.getMinimumBalanceForRentExemption(infoAccountSpace);

            await provider.sendAndConfirm(
                (() => {
                    const tx = new Transaction();
                    tx.add(
                        SystemProgram.createAccount({
                            fromPubkey: creator.publicKey,
                            newAccountPubkey: infoAccount.publicKey,
                            lamports: infoAccountLamports,
                            space: infoAccountSpace,
                            programId: swap.golanaLoader.programId,
                        })
                    );
                    return tx;
                })(),
                [creator, infoAccount],
                { skipPreflight: true },
            );

            mintA = await createMint(
                provider.connection,
                creator,
                mintAuthority.publicKey,
                null,
                15
            );

            mintB = await createMint(
                provider.connection,
                creator,
                mintAuthority.publicKey,
                null,
                15
            );

            mintLP = await createMint(
                provider.connection,
                creator,
                lp_token_mint_auth_pda,
                null,
                15
            );

            depositorTokenAccountA = await getOrCreateAssociatedTokenAccount(
                provider.connection,
                depositor,
                mintA,
                depositor.publicKey,
            );
            depositorTokenAccountB = await getOrCreateAssociatedTokenAccount(
                provider.connection,
                depositor,
                mintB,
                depositor.publicKey,
            );
            depositorTokenAccountLP = await getOrCreateAssociatedTokenAccount(
                provider.connection,
                depositor,
                mintLP,
                depositor.publicKey,
            );

            traderTokenAccountA = await getOrCreateAssociatedTokenAccount(
                provider.connection,
                trader,
                mintA,
                trader.publicKey,
            );
            traderTokenAccountB = await getOrCreateAssociatedTokenAccount(
                provider.connection,
                trader,
                mintB,
                trader.publicKey,
            );

            await mintTo(
                provider.connection,
                creator,
                mintA,
                depositorTokenAccountA.address,
                mintAuthority.publicKey,
                100000000,
                [mintAuthority],
            );

            await mintTo(
                provider.connection,
                creator,
                mintB,
                depositorTokenAccountB.address,
                mintAuthority.publicKey,
                100000000,
                [mintAuthority],
            );

            await mintTo(
                provider.connection,
                creator,
                mintA,
                traderTokenAccountA.address,
                mintAuthority.publicKey,
                1000,
                [mintAuthority],
            );

            await mintTo(
                provider.connection,
                creator,
                mintB,
                traderTokenAccountB.address,
                mintAuthority.publicKey,
                2000,
                [mintAuthority],
            );

            console.log(creator.publicKey.toString());
            console.log(vault_authority_pda.toString());
            console.log(infoAccount.publicKey.toString());
            console.log(mintA.toString());
            console.log(mintB.toString());

        });

        it("IxCreatePool", async () => {
            await swap.methods
                .IxCreatePool(new BN(10000), new BN(100))
                .accounts({
                    creator: creator.publicKey,
                    mintA: mintA,
                    mintB: mintB,
                    tokenAVault: vaultA.publicKey,
                    tokenBVault: vaultB.publicKey,
                    poolInfo: infoAccount.publicKey,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    rent: SYSVAR_RENT_PUBKEY,
                })
                .signers([creator, vaultA, vaultB])
                .rpc({ skipPreflight: true });

            const _vaultA = await getAccount(provider.connection, vaultA.publicKey);
            // Check that the new owner is the PDA.
            assert.ok(_vaultA.owner.equals(vault_authority_pda));

            const _vaultB = await getAccount(provider.connection, vaultB.publicKey);
            // Check that the new owner is the PDA.
            assert.ok(_vaultB.owner.equals(vault_authority_pda));
        });

        it("IxDeposit", async () => {
            await swap.methods
                .IxDeposit(new BN(100000), new BN(400000), lp_token_mint_auth_bump)
                .accounts({
                    depositor: depositor.publicKey,
                    mintLiquidity: mintLP,
                    mintLpAuth: lp_token_mint_auth_pda,
                    tokenA: depositorTokenAccountA.address,
                    tokenB: depositorTokenAccountB.address,
                    tokenLiquidity: depositorTokenAccountLP.address,
                    tokenAVault: vaultA.publicKey,
                    tokenBVault: vaultB.publicKey,
                    vaultAuthority: vault_authority_pda,
                    poolInfo: infoAccount.publicKey,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                })
                .signers([depositor])
                .rpc({ skipPreflight: true });

            const _depositorA = await getAccount(provider.connection, depositorTokenAccountA.address);
            console.log("depositorA", _depositorA.amount.toString());
            const _depositorB = await getAccount(provider.connection, depositorTokenAccountB.address);
            console.log("depositorB", _depositorB.amount.toString());
            const _lpAccount = await getAccount(provider.connection, depositorTokenAccountLP.address);
            console.log("depositorLP", _lpAccount.amount.toString());

            const _vaultA = await getAccount(provider.connection, vaultA.publicKey);
            console.log("vaultA", _vaultA.amount.toString());
            const _vaultB = await getAccount(provider.connection, vaultB.publicKey);
            console.log("vaultB", _vaultB.amount.toString());
        });

        it("IxTrade", async () => {
            const _traderA1 = await getAccount(provider.connection, traderTokenAccountA.address);
            console.log("before trade, A:", _traderA1.amount.toString());
            const _traderB1 = await getAccount(provider.connection, traderTokenAccountB.address);
            console.log("before trade, B:", _traderB1.amount.toString());
            await swap.methods
                .IxTrade(new BN(100), new BN(200), vault_authority_bump)
                .accounts({
                    trader: trader.publicKey,
                    tokenA: traderTokenAccountA.address,
                    tokenB: traderTokenAccountB.address,
                    tokenAVault: vaultA.publicKey,
                    tokenBVault: vaultB.publicKey,
                    vaultAuthority: vault_authority_pda,
                    poolInfo: infoAccount.publicKey,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .signers([trader])
                .rpc({ skipPreflight: true });

            const _traderA = await getAccount(provider.connection, traderTokenAccountA.address);
            console.log("After trade, A:", _traderA.amount.toString());
            const _traderB = await getAccount(provider.connection, traderTokenAccountB.address);
            console.log("After trade, B:", _traderB.amount.toString());
        });

        it("IxWithdraw", async () => {
            await swap.methods
                .IxWithdraw(new BN(100000), vault_authority_bump)
                .accounts({
                    depositor: depositor.publicKey,
                    mintLiquidity: mintLP,
                    mintLpAuth: lp_token_mint_auth_pda,
                    tokenA: depositorTokenAccountA.address,
                    tokenB: depositorTokenAccountB.address,
                    tokenLiquidity: depositorTokenAccountLP.address,
                    tokenAVault: vaultA.publicKey,
                    tokenBVault: vaultB.publicKey,
                    vaultAuthority: vault_authority_pda,
                    poolInfo: infoAccount.publicKey,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .signers([depositor])
                .rpc({ skipPreflight: true });

            const _depositorA = await getAccount(provider.connection, depositorTokenAccountA.address);
            console.log("depositorA", _depositorA.amount.toString());
            const _depositorB = await getAccount(provider.connection, depositorTokenAccountB.address);
            console.log("depositorB", _depositorB.amount.toString());
            const _lpAccount = await getAccount(provider.connection, depositorTokenAccountLP.address);
            console.log("depositorLP", _lpAccount.amount.toString());

            const _vaultA = await getAccount(provider.connection, vaultA.publicKey);
            console.log("vaultA", _vaultA.amount.toString());
            const _vaultB = await getAccount(provider.connection, vaultB.publicKey);
            console.log("vaultB", _vaultB.amount.toString());
        });

        it("IxClosePool", async () => {
            await swap.methods
                .IxClosePool(vault_authority_bump)
                .accounts({
                    creator: creator.publicKey,
                    tokenAVault: vaultA.publicKey,
                    tokenBVault: vaultB.publicKey,
                    vaultAuthority: vault_authority_pda,
                    poolInfo: infoAccount.publicKey,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .signers([creator])
                .rpc({ skipPreflight: true });
        });

    } catch (e) {
        console.error(e);
    }
});
