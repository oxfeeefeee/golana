import { PublicKey, AccountMeta, Signer, TransactionInstruction, ConfirmOptions } from "@solana/web3.js";
import { Program as AnchorProgram, Provider, getProvider, utils, Address, Accounts } from "@project-serum/anchor";
import { BinaryWriter } from 'borsh';
import { IDL as LoaderIDL, Loader } from "./loader.js";
import { Idl, IdlInstruction, IdlAccountItem, IdlAccounts, isIdlAccounts } from "./idl.js";
import { AllInstructions, MethodsFn, MakeMethodsNamespace, ArgsTuple, IdlTypes } from './types.js';

const LOADER_ID = "6ZjLk7jSFVVb2rxeoRf4ex3Q7zECi5SRTV4HbX55nNdP";

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
   * Address of the program.
   */
  public get programId(): PublicKey {
    return this._programId;
  }
  private _programId: PublicKey;

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
  private _idl: IDL;

  /**
   * Wallet and network provider.
   */
  public get provider(): Provider {
    return this._provider;
  }
  private _provider: Provider;

  public get bytecodePK(): Promise<PublicKey> {
    return this._bytecodePK;
  }
  private _bytecodePK: Promise<PublicKey>;

  public constructor(
    idl: IDL,
    programId: Address,
    seed: string,
    provider = getProvider(),
    golanaLoaderId: Address = LOADER_ID,
  ) {
    programId = translateAddress(programId);
    golanaLoaderId = translateAddress(golanaLoaderId);

    // Fields.
    this._idl = idl;
    this._provider = provider;
    this._programId = programId;
    this._golanaLoader = new AnchorProgram<Loader>(LoaderIDL, golanaLoaderId);
    this._bytecodePK = PublicKey.createWithSeed(provider.publicKey as PublicKey, seed, golanaLoaderId);

    this.methods = Object.fromEntries(
      idl.instructions.map(idlIx => [
        idlIx.name,
        MethodsBuilderFactory.build<IDL, typeof idlIx>(
          this._golanaLoader,
          this._bytecodePK,
          idlIx
        )
      ])
    ) as unknown as MethodsNamespace<IDL>;
  }

  async findAddr(seed: string) {
    let buf: Buffer | Uint8Array;
    const bytecodePK = await this._bytecodePK;

    if (typeof crypto !== "undefined") {
      const bytecodeBuf = bytecodePK.toBytes();
      const seedBuf = utils.bytes.utf8.encode(seed);
      const fullSeed = new Uint8Array(bytecodeBuf.length + seedBuf.length);
      fullSeed.set(bytecodeBuf);
      fullSeed.set(seedBuf, bytecodeBuf.length);
      const hash = await crypto.subtle.digest("SHA-256", fullSeed);
      buf = new Uint8Array(hash);
    } else {
      const crypto = await import("node:crypto");
      const fullSeed = Buffer.concat([bytecodePK.toBuffer(), Buffer.from(utils.bytes.utf8.encode(seed))]);
      buf = crypto.createHash("sha256").update(fullSeed).digest();
    }

    return PublicKey.findProgramAddressSync([buf], this._golanaLoader.programId);
  }
}

export class MethodsBuilderFactory {
  public static build<IDL extends Idl, I extends AllInstructions<IDL>>(
    loader: AnchorProgram<Loader>,
    bytecodePK: Promise<PublicKey>,
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
    private _bytecodePK: Promise<PublicKey>,
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

    return this._exec.remainingAccounts(accs);
  }

  public accountsStrict(
    accounts: Accounts<I["accounts"][number]>
  ) {
    return this._exec.accountsStrict(accounts);
  }

  public signers(signers: Array<Signer>) {
    return this._exec.signers(signers);
  }

  public remainingAccounts(
    accounts: Array<AccountMeta>
  ) {
    return this._exec.remainingAccounts(accounts);
  }

  public preInstructions(
    ixs: Array<TransactionInstruction>
  ) {
    return this._exec.preInstructions(ixs);
  }

  public postInstructions(
    ixs: Array<TransactionInstruction>
  ) {
    return this._exec.postInstructions(ixs);
  }

  public async rpc(options?: ConfirmOptions) {
    const bytecode = await this._bytecodePK;
    return this._exec.accounts({ bytecode }).rpc(options);
  }

  public async rpcAndKeys(options?: ConfirmOptions) {
    const bytecode = await this._bytecodePK;
    return this._exec.accounts({ bytecode }).rpcAndKeys(options);
  }

  public async view(options?: ConfirmOptions) {
    const bytecode = await this._bytecodePK;
    return this._exec.accounts({ bytecode }).view(options);
  }

  public async simulate(
    options?: ConfirmOptions
  ) {
    const bytecode = await this._bytecodePK;
    return this._exec.accounts({ bytecode }).simulate(options);
  }

  public async instruction() {
    const bytecode = await this._bytecodePK;
    return this._exec.accounts({ bytecode }).instruction();
  }

  /**
   * Convenient shortcut to get instructions and pubkeys via
   * const { pubkeys, instructions } = await prepare();
   */
  public async prepare() {
    const bytecode = await this._bytecodePK;
    return this._exec.accounts({ bytecode }).prepare();
  }

  public async transaction() {
    const bytecode = await this._bytecodePK;
    return this._exec.accounts({ bytecode }).transaction();
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
