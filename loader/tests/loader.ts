import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, SystemProgram, Transaction, Connection, Commitment, ComputeBudgetProgram, AccountMeta } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { Loader } from "../target/types/loader";
import { assert } from "chai";
import { serialize, BinaryWriter } from 'borsh';
import { sha256 } from 'js-sha256';

describe("loader", async () => {
    const provider = anchor.AnchorProvider.local();

    // Configure the client to use the local cluster.
    anchor.setProvider(provider);

    const golanaProgram = anchor.workspace.Loader as Program<Loader>;

    // Author for the tests.
    //const author = anchor.web3.Keypair.generate();
    // Create Golana account
    const seed = "escrow";
    const bytecodePK = await anchor.web3.PublicKey.createWithSeed(
        provider.wallet.publicKey, seed, golanaProgram.programId
    );

    // it("Create", async () => {
    //     // Airdropping tokens to author.
    //     const latestBlockhash = await provider.connection.getLatestBlockhash();
    //     const sig = await provider.connection.requestAirdrop(author.publicKey, 1000000000);
    //     await provider.connection.confirmTransaction(
    //         { signature: sig, ...latestBlockhash },
    //         "processed"
    //     );

    //     await provider.sendAndConfirm(
    //         (() => {
    //             const tx = new anchor.web3.Transaction();
    //             tx.add(
    //                 anchor.web3.SystemProgram.createAccountWithSeed({
    //                     fromPubkey: author.publicKey,
    //                     newAccountPubkey: bytecodePK,
    //                     basePubkey: author.publicKey,
    //                     seed: seed,
    //                     lamports: 900000000,
    //                     space: 32 * 1024,
    //                     programId: golanaProgram.programId,
    //                 })
    //             );
    //             return tx;
    //         })(),
    //         [author],
    //         //{ skipPreflight: true },
    //     );

    // });


    // it("Initialize", async () => {
    //     await golanaProgram.methods.golInitialize(seed).accounts({
    //         authority: author.publicKey,
    //         bytecode: bytecodePK,
    //     }).signers([author]).rpc({ skipPreflight: true });

    //     let bcAccount = await golanaProgram.account.golBytecode.fetch(bytecodePK);
    //     console.log(bcAccount);

    //     assert.ok(bcAccount.authority.equals(author.publicKey));
    //     assert.ok(bcAccount.handle == seed);
    //     assert.ok(bcAccount.content.length == 0);
    //     assert.ok(!bcAccount.finalized);
    // });


    // it("Write", async () => {
    //     const fs = require('fs');
    //     const content = fs.readFileSync('../examples/escrow/target/escrow.gosb');
    //     console.log("content length: ", content.length);
    //     const size = 850;
    //     let totalSent = 0;

    //     while (totalSent < content.length) {
    //         const end = Math.min(totalSent + size, content.length);
    //         const data = content.slice(totalSent, end);

    //         await golanaProgram.methods.golWrite(data).accounts({
    //             authority: author.publicKey,
    //             bytecode: bytecodePK,
    //         }).preInstructions(
    //             [
    //                 ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 })]
    //             // ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })]
    //         ).signers([author]).rpc({ skipPreflight: true });
    //         totalSent = end;

    //         console.log("Total sent: ", totalSent);
    //     }

    //     let bcAccount = await golanaProgram.account.golBytecode.fetch(bytecodePK);
    //     assert.ok(bcAccount.content.length == content.length);
    // });


    // it("Finalize", async () => {
    //     await golanaProgram.methods.golFinalize().accounts({
    //         authority: author.publicKey,
    //         bytecode: bytecodePK,
    //     }).preInstructions(
    //         [
    //             ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
    //             ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })]
    //     ).signers([author]).rpc({ skipPreflight: true });

    //     let bcAccount = await golanaProgram.account.golBytecode.fetch(bytecodePK);
    //     assert.ok(bcAccount.finalized);
    // });

    // class Assignable {
    //     constructor(properties) {
    //         Object.keys(properties).map((key) => {
    //             this[key] = properties[key];
    //         });
    //     }
    // }

    // class Test extends Assignable { }

    // const value = new Test({ x: 100, y: 20, z: '123' });
    // const schema = new Map([[Test, { kind: 'struct', fields: [['x', 'u8'], ['y', 'u8'], ['z', 'string']] }]]);
    // const buffer = serialize(schema, value);
    // console.log(buffer);

    // let writer = new BinaryWriter();
    // writer.writeString("escrow dsfew");
    // writer.writeU32(123);
    // const buf = writer.toArray();
    // it("Execute", async () => exec([], buf));



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


    let exec = async (ixName: string, accounts: Array<AccountMeta>, args: Uint8Array, signers: Array<anchor.web3.Signer>) => {
        await golanaProgram.methods.golExecute(ixName, args).accounts({
            bytecode: bytecodePK,
        }).remainingAccounts(accounts).preInstructions(
            [
                ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
                ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })]
        ).signers(signers).rpc({ skipPreflight: true });

        let bcAccount = await golanaProgram.account.golBytecode.fetch(bytecodePK);
        assert.ok(bcAccount.finalized);
    }

    let findAddr = async (seed) => {
        let fullSeed = Buffer.concat([bytecodePK.toBuffer(), Buffer.from(anchor.utils.bytes.utf8.encode(seed))]);
        let result = await PublicKey.findProgramAddress(
            [new Uint8Array(sha256.arrayBuffer(fullSeed))],
            golanaProgram.programId
        );
        return result
    }


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
        await exec('IxInit', accounts, buf, [initializerMainAccount]);

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
        await exec("IxExchange", accounts, buf, [takerMainAccount]);
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
    //     await exec("IxCancel", accounts, buf, [initializerMainAccount]);
    // })

});
