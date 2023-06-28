use anchor_lang::prelude::*;
use go_vm::Bytecode;
use golana::*;
use malloc::DualMalloc;
use std::rc::Rc;

declare_id!("6ZjLk7jSFVVb2rxeoRf4ex3Q7zECi5SRTV4HbX55nNdP");

mod ffi;
mod goscript;
mod malloc;

#[cfg(target_os = "solana")]
#[global_allocator]
static ALLOC: DualMalloc = DualMalloc::new();

#[program]
pub mod loader {
    use super::*;

    pub fn gol_initialize(ctx: Context<GolInitialize>, handle: String) -> Result<()> {
        DualMalloc::set_use_smalloc(true);

        let mem_dump_key = ctx.accounts.mem_dump.key();
        let mem_dump = &mut ctx.accounts.mem_dump.load_init()?;
        initialize_impl(
            &ctx.accounts.authority,
            &mut ctx.accounts.bytecode,
            &mem_dump_key,
            mem_dump,
            handle,
        )
    }

    pub fn gol_clear(ctx: Context<GolClear>, handle: String) -> Result<()> {
        DualMalloc::set_use_smalloc(true);

        let mem_dump_key = ctx.accounts.mem_dump.key();
        let mem_dump = &mut ctx.accounts.mem_dump.load_mut()?;
        initialize_impl(
            &ctx.accounts.authority,
            &mut ctx.accounts.bytecode,
            &mem_dump_key,
            mem_dump,
            handle,
        )
    }

    pub fn gol_write(ctx: Context<GolWrite>, data: Vec<u8>) -> Result<()> {
        DualMalloc::set_use_smalloc(true);

        ctx.accounts.bytecode.content.extend(data.iter());
        Ok(())
    }

    pub fn gol_finalize(ctx: Context<GolFinalize>) -> Result<()> {
        DualMalloc::set_use_smalloc(true);
        let bc = Bytecode::try_from_slice(&ctx.accounts.bytecode.content).unwrap();
        let meta = golana::check(&bc)?;
        let bc_ptr = Rc::into_raw(Rc::new(bc)) as u64;
        let meta_ptr = Rc::into_raw(Rc::new(meta)) as u64;
        DualMalloc::set_use_smalloc(false);

        ctx.accounts.bytecode.finalized = true;

        // Now save the content of the memory used by smalloc.
        let mem_dump = &mut ctx.accounts.mem_dump.load_mut()?;
        let dump_ptr = &mut mem_dump.dump as *mut [u8] as *mut u8;
        let dump = unsafe { std::slice::from_raw_parts_mut(dump_ptr, DualMalloc::smalloc_size()) };
        dump.copy_from_slice(DualMalloc::smalloc_mem_as_slice());
        mem_dump.bc_ptr = bc_ptr;
        mem_dump.meta_ptr = meta_ptr;

        Ok(())
    }

    pub fn gol_execute(ctx: Context<GolExecute>, id: String, args: Vec<u8>) -> Result<()> {
        msg!(&id);
        let mem_dump = &mut ctx.accounts.mem_dump.load()?;
        // Load the content of the memory stored by gol_finalize.
        // Now bytecode is ready to use without deserialization!
        let dump_ptr = &mem_dump.dump as *const [u8] as *const u8;
        let dump = unsafe { std::slice::from_raw_parts(dump_ptr, DualMalloc::smalloc_size()) };
        DualMalloc::smalloc_mem_as_slice().copy_from_slice(&dump[..]);

        DualMalloc::set_use_smalloc(true);
        let bc = match Rc::try_unwrap(unsafe { Rc::from_raw(mem_dump.bc_ptr as *const Bytecode) }) {
            Ok(bc) => bc,
            Err(_) => panic!("Failed to convert ptr into a Bytecode"),
        };
        let meta = match Rc::try_unwrap(unsafe {
            Rc::from_raw(mem_dump.meta_ptr as *const golana::TxMeta)
        }) {
            Ok(meta) => meta,
            Err(_) => panic!("Failed to convert ptr into a TxMeta"),
        };
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

fn initialize_impl(
    auth: &Signer,
    bytecode: &mut Account<GolBytecode>,
    mem_dump_key: &Pubkey,
    mem_dump: &mut GolMemoryDump,
    handle: String,
) -> Result<()> {
    let seed = String::from("BC") + &handle;
    let bc_pk = Pubkey::create_with_seed(auth.key, &seed, &ID).unwrap();
    require!(bytecode.key() == bc_pk, GolError::WrongHandle);

    let seed = String::from("MM") + &handle;
    let mm_pk = Pubkey::create_with_seed(auth.key, &seed, &ID).unwrap();
    require!(mem_dump_key == &mm_pk, GolError::WrongHandle);

    bytecode.handle = handle;
    bytecode.authority = auth.key();
    bytecode.finalized = false;
    bytecode.content = vec![];

    mem_dump.bytecode = bc_pk;
    mem_dump.meta_ptr = 0;
    mem_dump.bc_ptr = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction(handle: String)]
pub struct GolInitialize<'info> {
    pub authority: Signer<'info>,
    #[account(zero)]
    pub bytecode: Account<'info, GolBytecode>,
    #[account(zero)]
    pub mem_dump: AccountLoader<'info, GolMemoryDump>,
}

#[derive(Accounts)]
#[instruction(handle: String)]
pub struct GolClear<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub bytecode: Account<'info, GolBytecode>,
    #[account(mut)]
    pub mem_dump: AccountLoader<'info, GolMemoryDump>,
}

#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct GolWrite<'info> {
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    pub bytecode: Account<'info, GolBytecode>,
}

#[derive(Accounts)]
pub struct GolFinalize<'info> {
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    pub bytecode: Account<'info, GolBytecode>,
    #[account(mut, has_one = bytecode)]
    pub mem_dump: AccountLoader<'info, GolMemoryDump>,
}

#[derive(Accounts)]
#[instruction(id: String, args: Vec<u8>)]
pub struct GolExecute<'info> {
    pub mem_dump: AccountLoader<'info, GolMemoryDump>,
}

#[account]
pub struct GolBytecode {
    pub handle: String,
    pub authority: Pubkey,
    pub finalized: bool,
    pub content: Vec<u8>,
}

#[account(zero_copy)]
pub struct GolMemoryDump {
    pub bytecode: Pubkey,
    pub meta_ptr: u64,
    pub bc_ptr: u64,
    // Anchor doesn't support this: pub dump: [u8; DualMalloc::smalloc_size()],
    // so we just use a dummy value, and transmute when using it.
    pub dump: [u8; 256],
}
