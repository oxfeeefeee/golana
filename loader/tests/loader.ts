import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ComputeBudgetProgram } from '@solana/web3.js';
import { Loader } from "../target/types/loader";
import { assert } from "chai";

describe("loader", async () => {
    const provider = anchor.AnchorProvider.local();

    // Configure the client to use the local cluster.
    anchor.setProvider(provider);

    const program = anchor.workspace.Loader as Program<Loader>;

    // Author for the tests.
    const author = anchor.web3.Keypair.generate();
    // Create Golana account
    const seed = "test2";
    const bytecodePK = await anchor.web3.PublicKey.createWithSeed(
        author.publicKey, seed, program.programId
    );


    it("Create", async () => {
        // Airdropping tokens to author.
        const latestBlockhash = await provider.connection.getLatestBlockhash();
        const sig = await provider.connection.requestAirdrop(author.publicKey, 1000000000);
        await provider.connection.confirmTransaction(
            { signature: sig, ...latestBlockhash },
            "processed"
        );

        await provider.sendAndConfirm(
            (() => {
                const tx = new anchor.web3.Transaction();
                tx.add(
                    anchor.web3.SystemProgram.createAccountWithSeed({
                        fromPubkey: author.publicKey,
                        newAccountPubkey: bytecodePK,
                        basePubkey: author.publicKey,
                        seed: seed,
                        lamports: 100000000,
                        space: 8 * 1024,
                        programId: program.programId,
                    })
                );
                return tx;
            })(),
            [author],
            //{ skipPreflight: true },
        );

    });


    it("Initialize", async () => {
        await program.methods.golInitialize(seed).accounts({
            authority: author.publicKey,
            bytecode: bytecodePK,
        }).signers([author]).rpc({ skipPreflight: true });

        let bcAccount = await program.account.golBytecode.fetch(bytecodePK);
        console.log(bcAccount);

        assert.ok(bcAccount.authority.equals(author.publicKey));
        assert.ok(bcAccount.handle == seed);
        assert.ok(bcAccount.content.length == 0);
        assert.ok(!bcAccount.finalized);
    });


    it("Write", async () => {
        const fs = require('fs');
        const content = fs.readFileSync('../examples/escrow/target/escrow.gosb');
        console.log("content length: ", content.length);
        const size = 850;
        let totalSent = 0;

        while (totalSent < content.length) {
            const end = Math.min(totalSent + size, content.length);
            const data = content.slice(totalSent, end);

            await program.methods.golWrite(data).accounts({
                authority: author.publicKey,
                bytecode: bytecodePK,
            }).preInstructions(
                [
                    ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 })]
                // ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })]
            ).signers([author]).rpc({ skipPreflight: true });
            totalSent = end;

            console.log("Total sent: ", totalSent);
        }

        let bcAccount = await program.account.golBytecode.fetch(bytecodePK);
        assert.ok(bcAccount.content.length == content.length);
    });


    it("Finalize", async () => {
        await program.methods.golFinalize().accounts({
            authority: author.publicKey,
            bytecode: bytecodePK,
        }).preInstructions(
            [
                ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 })]
        ).signers([author]).rpc({ skipPreflight: true });

        let bcAccount = await program.account.golBytecode.fetch(bytecodePK);
        assert.ok(bcAccount.finalized);
    });


    it("Execute", async () => {
        await program.methods.golExecute(Buffer.from('My name is Paul')).accounts({
            authority: author.publicKey,
            bytecode: bytecodePK,
            extra: bytecodePK,
        }).preInstructions(
            [
                ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
                ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })]
        ).signers([author]).rpc({ skipPreflight: true });

        let bcAccount = await program.account.golBytecode.fetch(bytecodePK);
        assert.ok(bcAccount.finalized);
    });

});
