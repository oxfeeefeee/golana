import { IDL, Swap } from '../target/swap_idl.js';
import { Program, initFromEnv } from "golana";
import { ComputeBudgetProgram, Keypair, SystemProgram, Transaction } from '@solana/web3.js';
import BN from 'bn.js';

describe("swap", async () => {
    try {
        let provider = initFromEnv();

        const hello = new Program<Swap>(IDL, await Program.createCodePubKeys("swap"));


        // ...

    } catch (e) {
        console.error(e);
    }
});
