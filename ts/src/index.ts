// This TS project is based on https://github.com/coral-xyz/anchor/tree/master/ts/packages/anchor/src/program
import { PublicKey, AccountMeta, Signer, TransactionInstruction, ConfirmOptions, Connection, Transaction, VersionedTransaction, ComputeBudgetProgram } from "@solana/web3.js";
import { Program as AnchorProgram, Provider, AnchorProvider, getProvider, utils, Address, Accounts } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import * as borsh from 'borsh';
import { IDL as LoaderIDL, Loader } from "./loader.js";
import { Idl, IdlInstruction, IdlAccountItem, IdlAccounts, isIdlAccounts, IdlType } from "./idl.js";
import { AllInstructions, MethodsFn, MakeMethodsNamespace, ArgsTuple, IdlTypes } from './types.js';
import { createHash } from "crypto";

let LOADER_ID = "---Not initialized!!!!---";

// re-export AnchorProvider
export { AnchorProvider };

export interface AnchorWallet {
  publicKey: PublicKey;
  signTransaction<T extends Transaction | VersionedTransaction>(transaction: T): Promise<T>;
  signAllTransactions<T extends Transaction | VersionedTransaction>(transactions: T[]): Promise<T[]>;
}

export function initFromEnv(): AnchorProvider {
  LOADER_ID = process.env.GOLANA_LOADER_ID as string;

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  return provider;
}

export function initProvider(
  connection: Connection, wallet: AnchorWallet,
  golanaLoaderId: string,
  opts: ConfirmOptions = anchor.AnchorProvider.defaultOptions()
): AnchorProvider {
  LOADER_ID = golanaLoaderId;

  const provider = new anchor.AnchorProvider(connection, wallet, opts);
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

  private _bytecodePK: PublicKey;

  private _memDumpPK: PublicKey;

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

  private constructor(
    private _idl: IDL,
    bytecodePKAndMemDumpPK: [PublicKey, PublicKey],
    private _provider: Provider,
    golanaLoaderId: Address,
  ) {
    this._bytecodePK = bytecodePKAndMemDumpPK[0];
    this._memDumpPK = bytecodePKAndMemDumpPK[1];
    this._golanaLoader = new AnchorProgram<Loader>(LoaderIDL, address2Pubkey(golanaLoaderId), _provider);

    this.methods = Object.fromEntries(
      _idl.instructions.map(idlIx => [
        idlIx.name,
        MethodsBuilderFactory.build<IDL, typeof idlIx>(
          this._golanaLoader,
          this._memDumpPK,
          idlIx
        )
      ])
    ) as unknown as MethodsNamespace<IDL>;
  }

  static async create<IDL extends Idl>(
    idl: IDL,
    programAuth: Address,
    golanaLoaderId: Address = LOADER_ID,
  ): Promise<Program<IDL>> {
    const program_pubkeys = await Program.createCodePubKeys(idl.name, programAuth);
    return new Program<IDL>(idl, program_pubkeys, getProvider(), golanaLoaderId);
  }
  
  static async createCodePubKeys(name: string, programAuth: Address, golanaLoaderId: Address = LOADER_ID): Promise<[PublicKey, PublicKey]> {
    let pk = address2Pubkey(programAuth);
    let addr = address2Pubkey(golanaLoaderId);
    return [await PublicKey.createWithSeed(pk, "BC" + name, addr), await PublicKey.createWithSeed(pk, "MM" + name, addr)];
  }

  async findAddr(seed: string) {
    let buf: Buffer | Uint8Array;

    const fullSeed = Buffer.concat([this._bytecodePK.toBuffer(), Buffer.from(utils.bytes.utf8.encode(seed))]);
    buf = createHash("sha256").update(fullSeed).digest();
    
    return PublicKey.findProgramAddressSync([buf], this._golanaLoader.programId);
  }
}

export class MethodsBuilderFactory {
  public static build<IDL extends Idl, I extends AllInstructions<IDL>>(
    loader: AnchorProgram<Loader>,
    memDumpPK: PublicKey,
    idlIx: AllInstructions<IDL>
  ): MethodsFn<IDL, I, MethodsBuilder<IDL, I>> {
    return (...args) =>
      new MethodsBuilder(
        loader,
        memDumpPK,
        idlIx,
        args
      );
  }
}

export class MethodsBuilder<IDL extends Idl, I extends AllInstructions<IDL>> {
  private _exec;

  constructor(
    loader: AnchorProgram<Loader>,
    private _memDumpPK: PublicKey,
    private _idlIx: IdlInstruction,
    args: ArgsTuple<I["args"], IdlTypes<IDL>>,
  ) {
    const exec = loader.methods.golExecute(_idlIx.name, this._argsBuffer(args));
    exec.preInstructions([
      ComputeBudgetProgram.requestHeapFrame({ bytes: 256 * 1024 }),
      ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 })
    ]);
    this._exec = exec;
  }

  private _argsBuffer(args: ArgsTuple<I["args"], IdlTypes<IDL>>): Buffer {
    const buffers:Array<Uint8Array> = [];
    args.forEach((arg, i) => {
      const type = this._idlIx.args[i].type;
      const schema = getTypeSchema(type);
      if (type === "publicKey") {
        arg = (arg as PublicKey).toBytes();
      }
      buffers.push(borsh.serialize(schema, arg));
    });
    return Buffer.concat(buffers);
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

    this._exec.accounts({ memDump: this._memDumpPK }).remainingAccounts(accs);
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

// Convert an address to a Pubkey.
export function address2Pubkey(address: Address): PublicKey {
  return address instanceof PublicKey ? address : new PublicKey(address);
}

function getTypeSchema(idlType: IdlType): borsh.Schema {
  if (typeof idlType === 'string') {
    if ([
      "bool",
      "u8",
      "i8",
      "u16",
      "i16",
      "u32",
      "i32",
      "f32",
      "u64",
      "i64",
      "f64",
      "string",
    ].includes(idlType)) {
      return idlType as string;
    } else if (idlType === "bytes") {
      return {array:{type: "u8"}};
    } else if (idlType === "publicKey") {
      return {array:{type: "u8", len: 32}};
    } else {
      throw new Error(`Not a valid type: ${idlType}`);
    }
  } else if ("vec" in idlType) {
    return {array:{type: idlType.vec as string}};
  } else if ("array" in idlType) {
    return {array:{type: idlType.array[0] as string, len: idlType.array[1]}};
  } else {
    throw new Error(`Not a valid type: ${idlType}`);
  }
}
