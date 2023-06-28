import { IDL, Swap } from '../target/swap_idl.js';
import { Program, initFromEnv } from "golana";
import { ComputeBudgetProgram, Keypair, SystemProgram, Transaction, PublicKey, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import BN from 'bn.js';
import { assert } from "chai";

describe("swap", async () => {
    try {
        let provider = initFromEnv();

        const swap = new Program<Swap>(IDL, await Program.createCodePubKeys("swap"));

        const infoAccountSpace = 512;

        const infoAccount = Keypair.generate();
        const creator = Keypair.generate();
        const mintAuthority = Keypair.generate();

        let mintA: PublicKey;
        let mintB: PublicKey;

        let vault_account_a_pda: PublicKey;
        let vault_account_a_bump: number;
        let vault_account_b_pda: PublicKey;
        let vault_account_b_bump: number;
        let vault_authority_pda: PublicKey;
        let vault_authority_bump: number;


        it("Initialize program state", async () => {
            ([vault_account_a_pda, vault_account_a_bump] = await swap.findAddr("token-a"));
            ([vault_account_b_pda, vault_account_b_bump] = await swap.findAddr("token-b"));
            ([vault_authority_pda, vault_authority_bump] = await swap.findAddr("auth"));

            // Airdropping tokens to a creator.
            await provider.connection.confirmTransaction(
                await provider.connection.requestAirdrop(creator.publicKey, 1000000000),
                "processed"
            );

            const infoAccountLamports = await provider.connection.getMinimumBalanceForRentExemption(infoAccountSpace);

            // Fund Main Accounts
            await provider.sendAndConfirm(
                (() => {
                    const tx = new Transaction();
                    tx.add(
                        // SystemProgram.transfer({
                        //     fromPubkey: creator.publicKey,
                        //     toPubkey: initializerMainAccount.publicKey,
                        //     lamports: 100000000,
                        // }),
                        // SystemProgram.transfer({
                        //     fromPubkey: creator.publicKey,
                        //     toPubkey: takerMainAccount.publicKey,
                        //     lamports: 100000000,
                        // }),
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

            // initializerTokenAccountA = await createAccount(
            //     provider.connection,
            //     creator,
            //     mintA,
            //     initializerMainAccount.publicKey
            // );

            // initializerTokenAccountB = await createAccount(
            //     provider.connection,
            //     creator,
            //     mintB,
            //     initializerMainAccount.publicKey
            // );

            // takerTokenAccountA = await createAccount(
            //     provider.connection,
            //     creator,
            //     mintA,
            //     takerMainAccount.publicKey
            // );

            // takerTokenAccountB = await createAccount(
            //     provider.connection,
            //     creator,
            //     mintB,
            //     takerMainAccount.publicKey
            // );

            // await mintTo(
            //     provider.connection,
            //     creator,
            //     mintA,
            //     initializerTokenAccountA,
            //     mintAuthority.publicKey,
            //     initializerAmount,
            //     [mintAuthority],
            // );

            // await mintTo(
            //     provider.connection,
            //     creator,
            //     mintB,
            //     takerTokenAccountB,
            //     mintAuthority.publicKey,
            //     takerAmount,
            //     [mintAuthority],
            // );

            console.log(creator.publicKey.toString());
            console.log(vault_account_a_pda.toString(), vault_account_a_bump);
            console.log(vault_account_b_pda.toString(), vault_account_b_bump);
            console.log(vault_authority_pda.toString());
            console.log(infoAccount.publicKey.toString());
            console.log(mintA.toString());
            console.log(mintB.toString());

        });

        it("IxCreatePool", async () => {
            await swap.methods
                .IxCreatePool(new BN(100), vault_account_a_bump, vault_account_b_bump)
                .accounts({
                    creator: creator.publicKey,
                    mintA: mintA,
                    mintB: mintB,
                    tokenAVault: vault_account_a_pda,
                    tokenBVault: vault_account_b_pda,
                    poolInfo: infoAccount.publicKey,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    rent: SYSVAR_RENT_PUBKEY,
                })
                .preInstructions([
                    ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
                    ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 }),
                ])
                .signers([creator])
                .rpc({ skipPreflight: true });

            const _vaultA = await getAccount(provider.connection, vault_account_a_pda);
            // Check that the new owner is the PDA.
            assert.ok(_vaultA.owner.equals(vault_authority_pda));

            const _vaultB = await getAccount(provider.connection, vault_account_b_pda);
            // Check that the new owner is the PDA.
            assert.ok(_vaultB.owner.equals(vault_authority_pda));
        });

        it("IxClosePool", async () => {
            await swap.methods
                .IxClosePool(vault_authority_bump)
                .accounts({
                    creator: creator.publicKey,
                    tokenAVault: vault_account_a_pda,
                    tokenBVault: vault_account_b_pda,
                    vaultAuthority: vault_authority_pda,
                    poolInfo: infoAccount.publicKey,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .preInstructions([
                    ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
                    ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 }),
                ])
                .signers([creator])
                .rpc({ skipPreflight: true });

        });


    } catch (e) {
        console.error(e);
    }
});
