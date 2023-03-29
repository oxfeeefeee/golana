import { PublicKey, AccountMeta, Signer, TransactionInstruction, ConfirmOptions } from "@solana/web3.js";
import { Program as AnchorProgram, Provider, getProvider, utils, Address, Accounts } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import { BinaryWriter } from 'borsh';
import { IDL as LoaderIDL, Loader } from "./loader.js";
import { Idl, IdlInstruction, IdlAccountItem, IdlAccounts, isIdlAccounts } from "./idl.js";
import { AllInstructions, MethodsFn, MakeMethodsNamespace, ArgsTuple, IdlTypes } from './types.js';

let LOADER_ID = "Not initialized!!!";

export function initFromEnv(): Provider {
  LOADER_ID = process.env.GOLANA_LOADER_ID as string;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  return provider;
}

export type MethodsNamespace<
  IDL extends Idl = Idl,
  I extends AllInstructions<IDL> = AllInstructions<IDL>
> = MakeMethodsNamespace<IDL, I>;

export type PartialAccounts<A extends IdlAccountItem = IdlAccountItem> =
  Partial<{
    [N in A["name"]]: PartialAccount<A & { name: N }>;
  }>;

type PartialAccount<A extends IdlAccountItem> = A extends IdlAccounts
  ? PartialAccounts<A["accounts"][number]>
  : A extends { isOptional: true }
  ? Address | null
  : Address;

export class Program<IDL extends Idl = Idl> {
  /**
   * The namespace provides a builder API for all APIs on the program.
   * This is an alternative to using namespace the other namespaces..
   */
  readonly methods: MethodsNamespace<IDL>;

  /**
   * Address of the Golana loader.
   */
  public get golanaLoader(): AnchorProgram<Loader> {
    return this._golanaLoader;
  }
  private _golanaLoader: AnchorProgram<Loader>;

  /**
   * IDL defining the program's interface.
   */
  public get idl(): IDL {
    return this._idl;
  }

  /**
   * Wallet and network provider.
   */
  public get provider(): Provider {
    return this._provider;
  }

  public get bytecodePK(): PublicKey {
    return this._bytecodePK;
  }

  public constructor(
    private _idl: IDL,
    private _bytecodePK: PublicKey,
    private _provider = getProvider(),
    golanaLoaderId: Address = LOADER_ID,
  ) {
    this._golanaLoader = new AnchorProgram<Loader>(LoaderIDL, translateAddress(golanaLoaderId), _provider);

    this.methods = Object.fromEntries(
      _idl.instructions.map(idlIx => [
        idlIx.name,
        MethodsBuilderFactory.build<IDL, typeof idlIx>(
          this._golanaLoader,
          _bytecodePK,
          idlIx
        )
      ])
    ) as unknown as MethodsNamespace<IDL>;
  }

  static createByteCodePubKey(name: string, provider = getProvider(), golanaLoaderId: Address = LOADER_ID) {
    return PublicKey.createWithSeed(provider.publicKey as PublicKey, name, translateAddress(golanaLoaderId));
  }

  async findAddr(seed: string) {
    let buf: Buffer | Uint8Array;

    if (typeof crypto !== "undefined") {
      const bytecodeBuf = this._bytecodePK.toBytes();
      const seedBuf = utils.bytes.utf8.encode(seed);
      const fullSeed = new Uint8Array(bytecodeBuf.length + seedBuf.length);
      fullSeed.set(bytecodeBuf);
      fullSeed.set(seedBuf, bytecodeBuf.length);
      const hash = await crypto.subtle.digest("SHA-256", fullSeed);
      buf = new Uint8Array(hash);
    } else {
      const crypto = await import("node:crypto");
      const fullSeed = Buffer.concat([this._bytecodePK.toBuffer(), Buffer.from(utils.bytes.utf8.encode(seed))]);
      buf = crypto.createHash("sha256").update(fullSeed).digest();
    }

    return PublicKey.findProgramAddressSync([buf], this._golanaLoader.programId);
  }
}

export class MethodsBuilderFactory {
  public static build<IDL extends Idl, I extends AllInstructions<IDL>>(
    loader: AnchorProgram<Loader>,
    bytecodePK: PublicKey,
    idlIx: AllInstructions<IDL>
  ): MethodsFn<IDL, I, MethodsBuilder<IDL, I>> {
    return (...args) =>
      new MethodsBuilder(
        loader,
        bytecodePK,
        idlIx,
        args
      );
  }
}

export class MethodsBuilder<IDL extends Idl, I extends AllInstructions<IDL>> {
  private _exec;

  constructor(
    loader: AnchorProgram<Loader>,
    private _bytecodePK: PublicKey,
    private _idlIx: IdlInstruction,
    args: ArgsTuple<I["args"], IdlTypes<IDL>>,
  ) {
    this._exec = loader.methods.golExecute(_idlIx.name, this._argsBuffer(args));
  }

  private _argsBuffer(args: ArgsTuple<I["args"], IdlTypes<IDL>>): Buffer {
    const writer = new BinaryWriter();

    args.forEach((arg, i) => {
      const type = this._idlIx.args[i].type;

      if (type === "u8") {
        writer.writeU8(arg as number);
      } else if (type === "u64") {
        writer.writeU64(arg as number);
      }
    });

    return writer.toArray() as Buffer;
  }

  public args(args: ArgsTuple<I["args"], IdlTypes<IDL>>): void {
    this._exec.args([this._argsBuffer(args)]);
  }

  public async pubkeys() {
    return this._exec.pubkeys();
  }

  public accounts(accounts: PartialAccounts<I["accounts"][number]>) {
    const accs: AccountMeta[] = [];

    for (const [name, pubkey] of Object.entries(accounts)) {
      const acc = this._idlIx.accounts.find(acc => acc.name === name);

      if (acc === undefined) {
        throw new Error(`unknown account ${name}`);
      }

      if (isIdlAccounts(acc)) {
        throw new Error("not implemented");
      } else {
        accs.push({
          pubkey: pubkey as PublicKey, // TODO: check
          isWritable: acc.isMut,
          isSigner: acc.isSigner
        });
      }
    }

    this._exec.accounts({ bytecode: this._bytecodePK }).remainingAccounts(accs);
    return this;
  }

  public accountsStrict(
    accounts: Accounts<I["accounts"][number]>
  ) {
    this._exec.accountsStrict(accounts);
    return this;
  }

  public signers(signers: Array<Signer>) {
    this._exec.signers(signers);
    return this;
  }

  public remainingAccounts(
    accounts: Array<AccountMeta>
  ) {
    this._exec.remainingAccounts(accounts);
    return this;
  }

  public preInstructions(
    ixs: Array<TransactionInstruction>
  ) {
    this._exec.preInstructions(ixs);
    return this;
  }

  public postInstructions(
    ixs: Array<TransactionInstruction>
  ) {
    this._exec.postInstructions(ixs);
    return this;
  }

  public rpc(options?: ConfirmOptions) {
    return this._exec.rpc(options);
  }

  public rpcAndKeys(options?: ConfirmOptions) {
    return this._exec.rpcAndKeys(options);
  }

  public view(options?: ConfirmOptions) {
    return this._exec.view(options);
  }

  public simulate(
    options?: ConfirmOptions
  ) {
    return this._exec.simulate(options);
  }

  public instruction() {
    return this._exec.instruction();
  }

  /**
   * Convenient shortcut to get instructions and pubkeys via
   * const { pubkeys, instructions } = await prepare();
   */
  public prepare() {
    return this._exec.prepare();
  }

  public transaction() {
    return this._exec.transaction();
  }
}

// Translates an address to a Pubkey.
export function translateAddress(address: Address): PublicKey {
  return address instanceof PublicKey ? address : new PublicKey(address);
}


// export async function exec(
//   golanaProgram: AnchorProgram<Loader>,
//   bytecodePK: PublicKey,
//   ixName: string,
//   accounts: AccountMeta[],
//   args: Uint8Array | Buffer,
//   signers: Signer[]
// ) {
//   await golanaProgram.methods.golExecute(ixName, <Buffer>args)
//     .accounts({ bytecode: bytecodePK })
//     .remainingAccounts(accounts)
//     .preInstructions([
//         ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
//         ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })
//     ])
//     .signers(signers)
//     .rpc({ skipPreflight: true });

//   return golanaProgram.account.golBytecode.fetch(bytecodePK);
// }
