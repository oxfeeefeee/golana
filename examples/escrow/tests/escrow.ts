import dns from "node:dns";
import { IDL, Escrow } from '../target/escrow_idl.js';
import { Program, initFromEnv } from "golana";
import { PublicKey, SystemProgram, Transaction, Keypair } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import BN from 'bn.js';
import { assert } from "chai";

dns.setDefaultResultOrder("ipv4first");

describe("escrow", async () => {
  try {
    const provider = initFromEnv();

    const escrow = await Program.create<Escrow>(IDL, provider.publicKey);

    const takerAmount = 1001;
    const initializerAmount = 502;
    const escrowAccountSpace = 512;

    const escrowAccount = Keypair.generate();
    const payer = Keypair.generate();
    const mintAuthority = Keypair.generate();
    const initializerMainAccount = Keypair.generate();
    const takerMainAccount = Keypair.generate();

    let mintA: PublicKey;
    let mintB: PublicKey;
    let initializerTokenAccountA: PublicKey;
    let initializerTokenAccountB: PublicKey;
    let takerTokenAccountA: PublicKey;
    let takerTokenAccountB: PublicKey;
    let vault_account_pda: PublicKey;
    let vault_account_bump: number;
    let vault_authority_pda: PublicKey;
    let vault_authority_bump: number;

    it("Initialize program state", async () => {
      // Airdropping tokens to a payer.
      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(payer.publicKey, 1000000000),
        "processed"
      );

      const escrowAccountLamports = await provider.connection.getMinimumBalanceForRentExemption(escrowAccountSpace);

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
            SystemProgram.createAccount({
              fromPubkey: payer.publicKey,
              newAccountPubkey: escrowAccount.publicKey,
              lamports: escrowAccountLamports,
              space: escrowAccountSpace,
              programId: escrow.golanaLoader.programId,
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

      initializerTokenAccountA = await createAccount(
        provider.connection,
        payer,
        mintA,
        initializerMainAccount.publicKey
      );

      initializerTokenAccountB = await createAccount(
        provider.connection,
        payer,
        mintB,
        initializerMainAccount.publicKey
      );

      takerTokenAccountA = await createAccount(
        provider.connection,
        payer,
        mintA,
        takerMainAccount.publicKey
      );

      takerTokenAccountB = await createAccount(
        provider.connection,
        payer,
        mintB,
        takerMainAccount.publicKey
      );

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

      const _initializerTokenAccountA = await getAccount(provider.connection, initializerTokenAccountA);
      const _takerTokenAccountB = await getAccount(provider.connection, takerTokenAccountB);

      assert.ok(Number(_initializerTokenAccountA.amount) == initializerAmount);
      assert.ok(Number(_takerTokenAccountB.amount) == takerAmount);
    });

    it("Initialize escrow", async () => {
      ([vault_account_pda, vault_account_bump] = await escrow.findAddr("token-seed"));
      ([vault_authority_pda, vault_authority_bump] = await escrow.findAddr("escrow"));

      console.log(initializerMainAccount.publicKey.toString());
      console.log(vault_account_pda.toString(), vault_account_bump);
      console.log(vault_authority_pda.toString());
      console.log(initializerTokenAccountA.toString());
      console.log(escrowAccount.publicKey.toString());

      await escrow.methods.IxInit(
        vault_account_bump,
        new BN(initializerAmount),
        new BN(takerAmount)
      )
        .accounts({
          initializer: initializerMainAccount.publicKey,
          mint: mintA,
          vaultAccount: vault_account_pda,
          initializerDepositTokenAccount: initializerTokenAccountA,
          initializerReceiveTokenAccount: initializerTokenAccountB,
          escrowAccount: escrowAccount.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([initializerMainAccount])
        .rpc({ skipPreflight: true });

      const _vault = await getAccount(provider.connection, vault_account_pda);
      console.log(_vault.owner.toString());
      // Check that the new owner is the PDA.
      assert.ok(_vault.owner.equals(vault_authority_pda));
    });

    it("Exchange", async () => {
      await escrow.methods.IxExchange(vault_authority_bump)
        .accounts({
          taker: takerMainAccount.publicKey,
          takerDepositTokenAccount: takerTokenAccountB,
          takerReceiveTokenAccount: takerTokenAccountA,
          initializer: initializerMainAccount.publicKey,
          initializerDepositTokenAccount: initializerTokenAccountA,
          initializerReceiveTokenAccount: initializerTokenAccountB,
          escrowAccount: escrowAccount.publicKey,
          vaultAccount: vault_account_pda,
          vaultAuthority: vault_authority_pda,
          tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([takerMainAccount])
        .rpc({ skipPreflight: true });
    });

    // it("Cancel", async () => {
    //   const accounts = [
    //     {
    //       "pubkey": initializerMainAccount.publicKey,
    //       "isWritable": true,
    //       "isSigner": true
    //     },
    //     {
    //       "pubkey": initializerTokenAccountA,
    //       "isWritable": true,
    //       "isSigner": false
    //     },
    //     {
    //       "pubkey": vault_account_pda,
    //       "isWritable": true,
    //       "isSigner": false
    //     },
    //     {
    //       "pubkey": vault_authority_pda,
    //       "isWritable": false,
    //       "isSigner": false
    //     },
    //     {
    //       "pubkey": escrowAccount.publicKey,
    //       "isWritable": true,
    //       "isSigner": false
    //     },
    //     {
    //       "pubkey": TOKEN_PROGRAM_ID,
    //       "isWritable": false,
    //       "isSigner": false
    //     }
    //   ];

    //   let writer = new BinaryWriter();
    //   writer.writeU8(vault_authority_bump);
    //   const buf = writer.toArray();
    //   await exec("IxCancel", accounts, buf, [author, initializerMainAccount]);
    // });
  } catch (e) {
    console.error(e);
  }
});
