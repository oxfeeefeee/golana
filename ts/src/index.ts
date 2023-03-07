import { PublicKey, AccountMeta, Signer, ComputeBudgetProgram } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { IDL, Loader } from "./loader.js";
import address from "./address.js";

export function initGolana(cluster: "devnet" | "testnet" | "localnet") {
  const loaderAddress = new PublicKey(address[cluster]);
  return new Program<Loader>(IDL, loaderAddress);
}

export async function findAddr(golanaProgram: Program<Loader>, bytecodePK: PublicKey, seed: string) {
  let buf: Buffer | Uint8Array;

  if (typeof window !== "undefined" && globalThis.window === globalThis) {
    const bytecodeBuf = bytecodePK.toBytes();
    const seedBuf = anchor.utils.bytes.utf8.encode(seed);
    const fullSeed = new Uint8Array(bytecodeBuf.length + seedBuf.length);
    fullSeed.set(bytecodeBuf);
    fullSeed.set(seedBuf, bytecodeBuf.length);
    const hash = await crypto.subtle.digest("SHA-256", fullSeed);
    buf = new Uint8Array(hash);
  } else {
    const crypto = await import("node:crypto");
    const fullSeed = Buffer.concat([bytecodePK.toBuffer(), Buffer.from(anchor.utils.bytes.utf8.encode(seed))]);
    buf = crypto.createHash("sha256").update(fullSeed).digest();
  }

  return PublicKey.findProgramAddressSync([buf], golanaProgram.programId);
}

export async function exec(
  golanaProgram: Program<Loader>,
  bytecodePK: PublicKey,
  ixName: string,
  accounts: AccountMeta[],
  args: Uint8Array | Buffer,
  signers: Signer[]
) {
  await golanaProgram.methods.golExecute(ixName, <Buffer>args)
    .accounts({ bytecode: bytecodePK })
    .remainingAccounts(accounts)
    .preInstructions([
        ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
        ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })
    ])
    .signers(signers)
    .rpc({ skipPreflight: true });

  return golanaProgram.account.golBytecode.fetch(bytecodePK);
}
