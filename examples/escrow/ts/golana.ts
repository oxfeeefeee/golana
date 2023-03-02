// import * as anchor from "@project-serum/anchor";
// import { Program } from "@project-serum/anchor";
// import { PublicKey, SystemProgram, Transaction, Connection, Commitment, ComputeBudgetProgram, AccountMeta } from '@solana/web3.js';
// import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
// import { Loader } from "../target/types/loader";
// import { assert } from "chai";
// import { serialize, BinaryWriter } from 'borsh';
// import { sha256 } from 'js-sha256';



// const provider = anchor.AnchorProvider.local();

//     // Configure the client to use the local cluster.
//     anchor.setProvider(provider);

//     const golanaProgram = anchor.workspace.Loader as Program<Loader>;

//     // Author for the tests.
//     const author = anchor.web3.Keypair.generate();
//     // Create Golana account
//     const seed = "test2";
//     const bytecodePK = await anchor.web3.PublicKey.createWithSeed(
//         author.publicKey, seed, golanaProgram.programId
//     );



// let exec = async (ixName: string, accounts: Array<AccountMeta>, args: Uint8Array, signers: Array<anchor.web3.Signer>) => {
//     await golanaProgram.methods.golExecute(ixName, args).accounts({
//         authority: author.publicKey,
//         bytecode: bytecodePK,
//     }).remainingAccounts(accounts).preInstructions(
//         [
//             ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
//             ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })]
//     ).signers(signers).rpc({ skipPreflight: true });

//     let bcAccount = await golanaProgram.account.golBytecode.fetch(bytecodePK);
//     assert.ok(bcAccount.finalized);
// }

// let findAddr = async (seed) => {
//     let fullSeed = Buffer.concat([bytecodePK.toBuffer(), Buffer.from(anchor.utils.bytes.utf8.encode(seed))]);
//     let result = await PublicKey.findProgramAddress(
//         [new Uint8Array(sha256.arrayBuffer(fullSeed))],
//         golanaProgram.programId
//     );
//     return result
// }
