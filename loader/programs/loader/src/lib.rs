use anchor_lang::prelude::*;
use go_vm::Bytecode;
use golana::*;
use malloc::DualMalloc;
use std::rc::Rc;

declare_id!("HE7R2wfjpgjHnxfA9bS6fSLJzm7nucFfBXQhhxTCWMZs");

mod ffi;
mod goscript;
mod malloc;

#[cfg(target_os = "solana")]
#[global_allocator]
static ALLOC: DualMalloc = DualMalloc::new();

const MAX_HANDLE_LEN: usize = 32;

#[derive(PartialEq)]
enum FinalizeStep {
    Init,
    DeserialMetas,
    DeserialFuncs,
    DeserialPkgs,
    DeserialMisc,
    DeserialFileSet,
    Check,
}

impl From<usize> for FinalizeStep {
    fn from(i: usize) -> Self {
        match i {
            0 => Self::Init,
            1 => Self::DeserialMetas,
            2 => Self::DeserialFuncs,
            3 => Self::DeserialPkgs,
            4 => Self::DeserialMisc,
            5 => Self::DeserialFileSet,
            6 => Self::Check,
            _ => panic!("Invalid finalize step"),
        }
    }
}

#[program]
pub mod loader {
    use super::*;
    use go_vm::types::{
        Binding4Runtime, FunctionKey, FunctionObjs, GosValue, Meta, MetadataObjs, OpIndex,
        PackageKey, PackageObjs,
    };

    pub fn gol_initialize(ctx: Context<GolInitialize>, handle: String) -> Result<()> {
        DualMalloc::set_use_smalloc(true);

        let bytecode = &mut ctx.accounts.bytecode.load_init()?;
        let mem_dump = &mut ctx.accounts.mem_dump.load_init()?;
        initialize_impl(
            &ctx.accounts.authority,
            bytecode,
            &ctx.accounts.bytecode.key(),
            mem_dump,
            &ctx.accounts.mem_dump.key(),
            handle,
        )
    }

    pub fn gol_clear(ctx: Context<GolClear>, handle: String, new_size: u64) -> Result<()> {
        DualMalloc::set_use_smalloc(true);

        // Realloc if the new size is larger than the current size.
        let size = new_size as usize;
        let bc_account: &AccountInfo = ctx.accounts.bytecode.as_ref();
        if bc_account.data_len() < size {
            let rent = Rent::get()?;
            let new_minimum_balance = rent.minimum_balance(size);
            let lamports_diff = new_minimum_balance.saturating_sub(bc_account.lamports());
            let funding_account: &AccountInfo = ctx.accounts.authority.as_ref();
            solana_program::program::invoke(
                &solana_program::system_instruction::transfer(
                    funding_account.key,
                    bc_account.key,
                    lamports_diff,
                ),
                &[
                    funding_account.clone(),
                    bc_account.clone(),
                    ctx.accounts.system_program.clone(),
                ],
            )?;
            bc_account.realloc(size, false)?;
        }

        let bytecode = &mut ctx.accounts.bytecode.load_mut()?;
        let mem_dump = &mut ctx.accounts.mem_dump.load_mut()?;
        initialize_impl(
            &ctx.accounts.authority,
            bytecode,
            &ctx.accounts.bytecode.key(),
            mem_dump,
            &ctx.accounts.mem_dump.key(),
            handle,
        )
    }

    pub fn gol_write(ctx: Context<GolWrite>, data: Vec<u8>) -> Result<()> {
        DualMalloc::set_use_smalloc(true);

        let bc = &mut ctx.accounts.bytecode.load_mut()?;
        let dest = &mut bc.content as *mut [u8] as *mut u8;
        let dest = unsafe { dest.offset(bc.content_size as isize) };
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), dest, data.len());
        }
        bc.content_size += data.len();

        Ok(())
    }

    pub fn gol_finalize(ctx: Context<GolFinalize>, step_num: usize) -> Result<()> {
        let mem_dump = &mut ctx.accounts.mem_dump.load_mut()?;
        let step: FinalizeStep = step_num.into();
        if step == FinalizeStep::Init {
            mem_dump.data_offset = 0;
        } else {
            if mem_dump.finished_steps + 1 != step_num {
                return Err(GolError::WrongFinalizeStep.into());
            }
            restore_memory(mem_dump)?;
        }

        DualMalloc::set_use_smalloc(true);
        let bc_data = bytecode_data(&ctx.accounts.bytecode)?;
        let mut bytes = &bc_data[mem_dump.data_offset..];
        match step {
            FinalizeStep::Init => {
                mem_dump.bc_ptr = 0;
                mem_dump.meta_ptr = 0;
            }
            FinalizeStep::DeserialMetas => {
                let metas = MetadataObjs::deserialize(&mut bytes)?;
                mem_dump.meta_objs_ptr = obj_to_ptr(metas);
            }
            FinalizeStep::DeserialFuncs => {
                let funcs = FunctionObjs::deserialize(&mut bytes)?;
                mem_dump.func_objs_ptr = obj_to_ptr(funcs);
            }
            FinalizeStep::DeserialPkgs => {
                let pkgs = PackageObjs::deserialize(&mut bytes)?;
                mem_dump.pkg_objs_ptr = obj_to_ptr(pkgs);
            }
            FinalizeStep::DeserialMisc => {
                let consts = Vec::<GosValue>::deserialize(&mut bytes)?;
                mem_dump.consts_ptr = obj_to_ptr(consts);
                let ifaces = Vec::<(Meta, Vec<Binding4Runtime>)>::deserialize(&mut bytes)?;
                mem_dump.ifaces_ptr = obj_to_ptr(ifaces);
                let indices = Vec::<Vec<OpIndex>>::deserialize(&mut bytes)?;
                mem_dump.indices_ptr = obj_to_ptr(indices);
                mem_dump.entry = usize::deserialize(&mut bytes)?;
                mem_dump.main_pkg = usize::deserialize(&mut bytes)?;
            }
            FinalizeStep::DeserialFileSet => {
                let metas: MetadataObjs = obj_from_ptr(mem_dump.meta_objs_ptr);
                let funcs: FunctionObjs = obj_from_ptr(mem_dump.func_objs_ptr);
                let pkgs: PackageObjs = obj_from_ptr(mem_dump.pkg_objs_ptr);
                let consts: Vec<GosValue> = obj_from_ptr(mem_dump.consts_ptr);
                let ifaces: Vec<(Meta, Vec<Binding4Runtime>)> = obj_from_ptr(mem_dump.ifaces_ptr);
                let indices: Vec<Vec<OpIndex>> = obj_from_ptr(mem_dump.indices_ptr);
                let entry: FunctionKey = mem_dump.entry.into();
                let main_pkg: PackageKey = mem_dump.main_pkg.into();
                let file_set = Option::<go_vm::parser::FileSet>::deserialize(&mut bytes)?;

                let bc = Bytecode::with_components(
                    metas, funcs, pkgs, consts, ifaces, indices, entry, main_pkg, file_set,
                );
                mem_dump.bc_ptr = obj_to_ptr(bc);
            }
            FinalizeStep::Check => {
                let bc: Bytecode = obj_from_ptr(mem_dump.bc_ptr);
                let meta = golana::check(&bc)?;
                mem_dump.bc_ptr = obj_to_ptr(bc);
                mem_dump.meta_ptr = obj_to_ptr(meta);
            }
        }
        mem_dump.data_offset = bc_data.len() - bytes.len();
        DualMalloc::set_use_smalloc(false);

        mem_dump.finished_steps = step_num;
        dump_memory(mem_dump)?;
        Ok(())
    }

    pub fn gol_execute(ctx: Context<GolExecute>, id: String, args: Vec<u8>) -> Result<()> {
        msg!(&id);
        let mem_dump = &mut ctx.accounts.mem_dump.load()?;
        restore_memory(mem_dump)?;

        DualMalloc::set_use_smalloc(true);
        let bc: Bytecode = obj_from_ptr(mem_dump.bc_ptr);
        let meta: TxMeta = obj_from_ptr(mem_dump.meta_ptr);
        crate::goscript::run(
            &mem_dump.bytecode,
            &bc,
            &meta,
            ctx.remaining_accounts,
            &id,
            args,
        )
    }
}

fn bytecode_data<'a>(account: &'a AccountLoader<'_, GolBytecode>) -> Result<&'a [u8]> {
    let bc_serialized = &mut account.load_mut()?;
    let addr = &bc_serialized.content as *const [u8] as *const u8;
    Ok(unsafe { std::slice::from_raw_parts(addr, bc_serialized.content_size) })
}

fn obj_to_ptr<T>(obj: T) -> usize {
    Rc::into_raw(Rc::new(obj)) as usize
}

fn obj_from_ptr<T>(ptr: usize) -> T {
    match Rc::try_unwrap(unsafe { Rc::from_raw(ptr as *const T) }) {
        Ok(obj) => obj,
        Err(_) => panic!("Failed to convert ptr into an object: {}", ptr),
    }
}

fn dump_memory(mem_dump: &mut GolMemoryDump) -> Result<()> {
    let dump_ptr = &mut mem_dump.dump as *mut [u8] as *mut u8;
    let dump = unsafe { std::slice::from_raw_parts_mut(dump_ptr, DualMalloc::smalloc_size()) };
    dump.copy_from_slice(DualMalloc::smalloc_mem_as_slice());
    Ok(())
}

fn restore_memory(mem_dump: &GolMemoryDump) -> Result<()> {
    let dump_ptr = &mem_dump.dump as *const [u8] as *const u8;
    let dump = unsafe { std::slice::from_raw_parts(dump_ptr, DualMalloc::smalloc_size()) };
    DualMalloc::smalloc_mem_as_slice().copy_from_slice(&dump[..]);
    Ok(())
}

fn initialize_impl(
    auth: &Signer,
    bytecode: &mut GolBytecode,
    bytecode_key: &Pubkey,
    mem_dump: &mut GolMemoryDump,
    mem_dump_key: &Pubkey,
    handle: String,
) -> Result<()> {
    let seed = String::from("BC") + &handle;
    let bc_pk = Pubkey::create_with_seed(auth.key, &seed, &ID).unwrap();
    require!(bytecode_key == &bc_pk, GolError::WrongHandle);

    let seed = String::from("MM") + &handle;
    let mm_pk = Pubkey::create_with_seed(auth.key, &seed, &ID).unwrap();
    require!(mem_dump_key == &mm_pk, GolError::WrongHandle);

    bytecode.handle = string_to_array(&handle)?;
    bytecode.authority = auth.key();
    bytecode.finalized = 0;
    bytecode.content_size = 0;
    bytecode.content = [0; 8];

    mem_dump.bytecode = bc_pk;
    mem_dump.meta_ptr = 0;
    mem_dump.bc_ptr = 0;

    Ok(())
}

/// Convert a string into a fixed size array of bytes. If the string is shorter than the array,
/// the remaining bytes are set to 0.
fn string_to_array(s: &str) -> Result<[u8; MAX_HANDLE_LEN]> {
    require!(s.len() <= MAX_HANDLE_LEN, GolError::HandleTooLong);
    let mut arr = [0u8; MAX_HANDLE_LEN];
    arr[..s.len()].copy_from_slice(s.as_bytes());
    Ok(arr)
}

#[derive(Accounts)]
#[instruction(handle: String)]
pub struct GolInitialize<'info> {
    pub authority: Signer<'info>,
    #[account(zero)]
    pub bytecode: AccountLoader<'info, GolBytecode>,
    #[account(zero)]
    pub mem_dump: AccountLoader<'info, GolMemoryDump>,
}

#[derive(Accounts)]
#[instruction(handle: String, new_size: u64)]
pub struct GolClear<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub bytecode: AccountLoader<'info, GolBytecode>,
    #[account(mut)]
    pub mem_dump: AccountLoader<'info, GolMemoryDump>,
    /// CHECK: system_program is required to transfer lamports to the bytecode account.
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct GolWrite<'info> {
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    pub bytecode: AccountLoader<'info, GolBytecode>,
}

#[derive(Accounts)]
#[instruction(step: usize)]
pub struct GolFinalize<'info> {
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    pub bytecode: AccountLoader<'info, GolBytecode>,
    #[account(mut, has_one = bytecode)]
    pub mem_dump: AccountLoader<'info, GolMemoryDump>,
}

#[derive(Accounts)]
#[instruction(id: String, args: Vec<u8>)]
pub struct GolExecute<'info> {
    pub mem_dump: AccountLoader<'info, GolMemoryDump>,
}

#[account(zero_copy)]
pub struct GolBytecode {
    pub handle: [u8; MAX_HANDLE_LEN],
    pub authority: Pubkey,
    pub finalized: usize,
    pub content_size: usize,
    // a dummy size, and transmute when using it.
    pub content: [u8; 8],
}

#[account(zero_copy)]
pub struct GolMemoryDump {
    pub bytecode: Pubkey,
    pub meta_ptr: usize,
    pub bc_ptr: usize,
    // Below are temporary fields for deserializing the bytecode.
    pub finished_steps: usize,
    pub data_offset: usize,
    pub meta_objs_ptr: usize,
    pub func_objs_ptr: usize,
    pub pkg_objs_ptr: usize,
    pub consts_ptr: usize,
    pub ifaces_ptr: usize,
    pub indices_ptr: usize,
    pub entry: usize,
    pub main_pkg: usize,

    // Anchor doesn't support this: pub dump: [u8; DualMalloc::smalloc_size()],
    // so we just use a dummy value, and transmute when using it.
    pub dump: [u8; 256],
}
