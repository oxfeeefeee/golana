import dns from "node:dns";
import * as anchor from "@project-serum/anchor";
import { initGolana, findAddr, exec } from "golana";
import { PublicKey, SystemProgram, Transaction, AccountMeta } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { assert } from "chai";
import { BinaryWriter } from 'borsh';

dns.setDefaultResultOrder("ipv4first");

describe("escrow", async() => {
  try {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    // Initialize the golana program.
    const golana = initGolana("localnet");

    const authorPK = provider.publicKey;
    const seed = "escrow";
    const bytecodePK = await anchor.web3.PublicKey.createWithSeed(authorPK, seed, golana.programId);

    const takerAmount = 1001;
    const initializerAmount = 502;
    const escrowAccountSpace = 512;

    const escrowAccount = anchor.web3.Keypair.generate();
    const payer = anchor.web3.Keypair.generate();
    const mintAuthority = anchor.web3.Keypair.generate();
    const initializerMainAccount = anchor.web3.Keypair.generate();
    const takerMainAccount = anchor.web3.Keypair.generate();

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
            anchor.web3.SystemProgram.createAccount({
              fromPubkey: payer.publicKey,
              newAccountPubkey: escrowAccount.publicKey,
              lamports: escrowAccountLamports,
              space: escrowAccountSpace,
              programId: golana.programId,
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
      ([vault_account_pda, vault_account_bump] = await findAddr(golana, bytecodePK, "token-seed"));
      ([vault_authority_pda, vault_authority_bump] = await findAddr(golana, bytecodePK, "escrow"));

      const accounts: AccountMeta[] = [
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

      const writer = new BinaryWriter();
      writer.writeU8(vault_account_bump);
      writer.writeU64(initializerAmount);
      writer.writeU64(takerAmount);
      const buf = writer.toArray();
      const result = await exec(golana, bytecodePK, 'IxInit', accounts, buf, [initializerMainAccount]);
      assert.ok(result.finalized);

      const _vault = await getAccount(provider.connection, vault_account_pda);
      console.log(_vault.owner.toString());
      // Check that the new owner is the PDA.
      assert.ok(_vault.owner.equals(vault_authority_pda));
    });

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

      const writer = new BinaryWriter();
      writer.writeU8(vault_authority_bump);
      const buf = writer.toArray();
      await exec(golana, bytecodePK, "IxExchange", accounts, buf, [takerMainAccount]);
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
