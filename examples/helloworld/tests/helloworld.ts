import { IDL, Helloworld } from '../target/helloworld_idl.js';
import { Program, initFromEnv } from "golana";
import { ComputeBudgetProgram, Keypair, SystemProgram, Transaction } from '@solana/web3.js';
import BN from 'bn.js';

describe("helloworld", async () => {
    try {
        const provider = initFromEnv();

        const hello = await Program.create<Helloworld>(IDL, provider);

        const userAccountSpace = 512;
        const userAccount = Keypair.generate();
        const payer = Keypair.generate();

        it("Initialize program state", async () => {
            // Airdropping tokens to a payer.
            const latestBlockHash = await provider.connection.getLatestBlockhash();
            const airdrop = await provider.connection.requestAirdrop(payer.publicKey, 100000000);
            await provider.connection.confirmTransaction({
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: airdrop,
            });

           const userAccountLamports = await provider.connection.getMinimumBalanceForRentExemption(userAccountSpace);

            // Create the user account
            const trans = await provider.sendAndConfirm(
                (() => {
                    const tx = new Transaction();
                    tx.add(
                        SystemProgram.createAccount({
                            fromPubkey: payer.publicKey,
                            newAccountPubkey: userAccount.publicKey,
                            lamports: userAccountLamports,
                            space: userAccountSpace,
                            programId: hello.golanaLoader.programId,
                        })
                    );
                    return tx;
                })(),
                [payer, userAccount],
                { skipPreflight: true },
            );

            // const result = await provider.connection.getTransaction(trans);
            // console.log(result)
        });

        it("IxInit", async () => {
            const trans = await hello.methods.IxInit(
                new BN(666),
            )
                .accounts({
                    user: payer.publicKey,
                    userAccount: userAccount.publicKey,
                })
                .preInstructions([
                    ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
                    ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })
                ])
                .signers([payer])
                .rpc();
            
            // const result = await provider.connection.getTransaction(trans);
            // console.log(result)
        });

        it("IxGreet", async () => {
            await hello.methods.IxGreet("best_chain_devs")
                .accounts({
                    user: payer.publicKey,
                    userAccount: userAccount.publicKey,
                })
                .preInstructions([
                    ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
                    ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })
                ])
                .signers([payer])
                .rpc({ skipPreflight: true });
        });

        it("IxGreet2", async () => {
            await hello.methods.IxGreet("best_chain_devs")
                .accounts({
                    user: payer.publicKey,
                    userAccount: userAccount.publicKey,
                })
                .preInstructions([
                    ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
                    ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })
                ])
                .signers([payer])
                .rpc();
        });

    } catch (e) {
        console.error(e);
    }
});
