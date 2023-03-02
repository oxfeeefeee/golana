import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, SystemProgram, Transaction, Connection, Commitment, ComputeBudgetProgram, AccountMeta } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { Loader } from "../target/types/loader";
import { assert } from "chai";
import { serialize, BinaryWriter } from 'borsh';
import { sha256 } from 'js-sha256';

describe("escrow", async () => {

    ////////////////////////////////////////////////////////////////////////
    // Set up escrow 
    let mintA = null;
    let mintB = null;
    let initializerTokenAccountA = null;
    let initializerTokenAccountB = null;
    let takerTokenAccountA = null;
    let takerTokenAccountB = null;
    let vault_account_pda = null;
    let vault_account_bump = null;
    let vault_authority_pda = null;
    let vault_authority_bump = null;

    const takerAmount = 1001;
    const initializerAmount = 502;
    const escrowAccountSpace = 512;

    const escrowAccount = anchor.web3.Keypair.generate();
    const payer = anchor.web3.Keypair.generate();
    const mintAuthority = anchor.web3.Keypair.generate();
    const initializerMainAccount = anchor.web3.Keypair.generate();
    const takerMainAccount = anchor.web3.Keypair.generate();

    it("Initialize program state", async () => {
        // Airdropping tokens to a payer.
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(payer.publicKey, 1000000000),
            "processed"
        );

        let escrowAccountLamports = await provider.connection.getMinimumBalanceForRentExemption(escrowAccountSpace);

        // Fund Main Accounts
        await provider.sendAndConfirm(
            (() => {
                const tx = new Transaction();
                tx.add(
                    SystemProgram.transfer({
                        fromPubkey: payer.publicKey,
                        toPubkey: initializerMainAccount.publicKey,
                        lamports: 100000000,
                    }),
                    SystemProgram.transfer({
                        fromPubkey: payer.publicKey,
                        toPubkey: takerMainAccount.publicKey,
                        lamports: 100000000,
                    }),
                    anchor.web3.SystemProgram.createAccount({
                        fromPubkey: payer.publicKey,
                        newAccountPubkey: escrowAccount.publicKey,
                        lamports: escrowAccountLamports,
                        space: escrowAccountSpace,
                        programId: golanaProgram.programId,
                    })
                );
                return tx;
            })(),
            [payer, escrowAccount],
            { skipPreflight: true },
        );

        mintA = await createMint(
            provider.connection,
            payer,
            mintAuthority.publicKey,
            null,
            15
        );

        mintB = await createMint(
            provider.connection,
            payer,
            mintAuthority.publicKey,
            null,
            15
        );

        initializerTokenAccountA = await createAccount(provider.connection,
            payer, mintA, initializerMainAccount.publicKey);
        takerTokenAccountA = await createAccount(provider.connection,
            payer, mintA, takerMainAccount.publicKey);

        initializerTokenAccountB = await createAccount(provider.connection,
            payer, mintB, initializerMainAccount.publicKey);
        takerTokenAccountB = await createAccount(provider.connection,
            payer, mintB, takerMainAccount.publicKey);

        await mintTo(
            provider.connection,
            payer,
            mintA,
            initializerTokenAccountA,
            mintAuthority.publicKey,
            initializerAmount,
            [mintAuthority],
        );

        await mintTo(
            provider.connection,
            payer,
            mintB,
            takerTokenAccountB,
            mintAuthority.publicKey,
            takerAmount,
            [mintAuthority],
        );

        let _initializerTokenAccountA = await getAccount(provider.connection, initializerTokenAccountA);
        let _takerTokenAccountB = await getAccount(provider.connection, takerTokenAccountB);

        assert.ok(Number(_initializerTokenAccountA.amount) == initializerAmount);
        assert.ok(Number(_takerTokenAccountB.amount) == takerAmount);
    });
    ////////////////////////////////////////////////////////////////////////


    ////////////////////////////////////////////////////////////////////////
    it("Initialize escrow", async () => {
        const [_vault_account_pda, _vault_account_bump] = await findAddr("token-seed");
        vault_account_pda = _vault_account_pda;
        vault_account_bump = _vault_account_bump;

        const [_vault_authority_pda, _vault_authority_bump] = await findAddr("escrow");
        vault_authority_pda = _vault_authority_pda;
        vault_authority_bump = _vault_authority_bump;

        const accounts = [
            {
                "pubkey": initializerMainAccount.publicKey,
                "isWritable": true,
                "isSigner": true
            },
            {
                "pubkey": mintA,
                "isWritable": false,
                "isSigner": false
            },
            {
                "pubkey": vault_account_pda,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": initializerTokenAccountA,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": initializerTokenAccountB,
                "isWritable": false,
                "isSigner": false
            },
            {
                "pubkey": escrowAccount.publicKey,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": anchor.web3.SystemProgram.programId,
                "isWritable": false,
                "isSigner": false
            },
            {
                "pubkey": anchor.web3.SYSVAR_RENT_PUBKEY,
                "isWritable": false,
                "isSigner": false
            },
            {
                "pubkey": TOKEN_PROGRAM_ID,
                "isWritable": false,
                "isSigner": false
            }
        ];

        console.log(initializerMainAccount.publicKey.toString());
        console.log(vault_account_pda.toString(), vault_account_bump);
        console.log(vault_authority_pda.toString());
        console.log(initializerTokenAccountA.toString());
        console.log(escrowAccount.publicKey.toString());

        let writer = new BinaryWriter();
        writer.writeU8(vault_account_bump);
        writer.writeU64(initializerAmount);
        writer.writeU64(takerAmount);
        const buf = writer.toArray();
        await exec('IxInit', accounts, buf, [author, initializerMainAccount]);

        let _vault = await getAccount(provider.connection, vault_account_pda);
        console.log(_vault.owner.toString());
        // Check that the new owner is the PDA.
        assert.ok(_vault.owner.equals(vault_authority_pda));
    })

    it("Exchange", async () => {
        const accounts = [
            {
                "pubkey": takerMainAccount.publicKey,
                "isWritable": false,
                "isSigner": true
            },
            {
                "pubkey": takerTokenAccountB,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": takerTokenAccountA,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": initializerMainAccount.publicKey,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": initializerTokenAccountA,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": initializerTokenAccountB,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": escrowAccount.publicKey,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": vault_account_pda,
                "isWritable": true,
                "isSigner": false
            },
            {
                "pubkey": vault_authority_pda,
                "isWritable": false,
                "isSigner": false
            },
            {
                "pubkey": TOKEN_PROGRAM_ID,
                "isWritable": false,
                "isSigner": false
            }
        ];

        let writer = new BinaryWriter();
        writer.writeU8(vault_authority_bump);
        const buf = writer.toArray();
        await exec("IxExchange", accounts, buf, [author, takerMainAccount]);
    })

    // it("Cancel", async () => {
    //     const accounts = [
    //         {
    //             "pubkey": initializerMainAccount.publicKey,
    //             "isWritable": true,
    //             "isSigner": true
    //         },
    //         {
    //             "pubkey": initializerTokenAccountA,
    //             "isWritable": true,
    //             "isSigner": false
    //         },
    //         {
    //             "pubkey": vault_account_pda,
    //             "isWritable": true,
    //             "isSigner": false
    //         },
    //         {
    //             "pubkey": vault_authority_pda,
    //             "isWritable": false,
    //             "isSigner": false
    //         },
    //         {
    //             "pubkey": escrowAccount.publicKey,
    //             "isWritable": true,
    //             "isSigner": false
    //         },
    //         {
    //             "pubkey": TOKEN_PROGRAM_ID,
    //             "isWritable": false,
    //             "isSigner": false
    //         }
    //     ];

    //     let writer = new BinaryWriter();
    //     writer.writeU8(vault_authority_bump);
    //     const buf = writer.toArray();
    //     await exec("IxCancel", accounts, buf, [author, initializerMainAccount]);
    // })

});
