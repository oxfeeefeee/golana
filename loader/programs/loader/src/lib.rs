use anchor_lang::prelude::*;
use golana::*;
use goscript_vm::Bytecode;

declare_id!("6ZjLk7jSFVVb2rxeoRf4ex3Q7zECi5SRTV4HbX55nNdP");

mod ffi;
mod goscript;

#[cfg(target_os = "solana")]
const HEAP_LENGTH_MAX: usize = 256 * 1024;

#[cfg(target_os = "solana")]
#[global_allocator]
static ALLOC: smalloc::Smalloc<
    { solana_program::entrypoint::HEAP_START_ADDRESS as usize },
    { HEAP_LENGTH_MAX as usize },
    64,
    1024,
> = smalloc::Smalloc::new();

#[program]
pub mod loader {
    use super::*;

    pub fn gol_initialize(ctx: Context<GolInitialize>, handle: String) -> Result<()> {
        initialize_impl(&ctx.accounts.authority, &mut ctx.accounts.bytecode, handle)
    }

    pub fn gol_clear(ctx: Context<GolClear>, handle: String) -> Result<()> {
        initialize_impl(&ctx.accounts.authority, &mut ctx.accounts.bytecode, handle)
    }

    pub fn gol_write(ctx: Context<GolWrite>, data: Vec<u8>) -> Result<()> {
        ctx.accounts.bytecode.content.extend(data.iter());
        Ok(())
    }

    pub fn gol_finalize(ctx: Context<GolFinalize>) -> Result<()> {
        let bc = Bytecode::try_from_slice(&ctx.accounts.bytecode.content).unwrap();
        let meta = golana::check(&bc)?;
        ctx.accounts.bytecode.meta = meta.try_to_vec().unwrap();
        ctx.accounts.bytecode.finalized = true;
        Ok(())
    }

    pub fn gol_execute(ctx: Context<GolExecute>, id: String, args: Vec<u8>) -> Result<()> {
        crate::goscript::run(
            &ctx.accounts.bytecode.key(),
            &ctx.accounts.bytecode.content,
            &ctx.accounts.bytecode.meta,
            ctx.remaining_accounts,
            &id,
            args,
        )
    }
}

fn initialize_impl(
    auth: &Signer,
    bytecode: &mut Account<GolBytecode>,
    handle: String,
) -> Result<()> {
    let gol_pk = Pubkey::create_with_seed(auth.key, &handle, &ID).unwrap();
    require!(bytecode.key() == gol_pk, GolError::WrongHandle);

    bytecode.handle = handle;
    bytecode.authority = auth.key();
    bytecode.finalized = false;
    bytecode.content = vec![];
    Ok(())
}

#[derive(Accounts)]
#[instruction(handle: String)]
pub struct GolInitialize<'info> {
    pub authority: Signer<'info>,
    #[account(zero)]
    pub bytecode: Account<'info, GolBytecode>,
}

#[derive(Accounts)]
#[instruction(handle: String)]
pub struct GolClear<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub bytecode: Account<'info, GolBytecode>,
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
}

#[derive(Accounts)]
#[instruction(id: String, args: Vec<u8>)]
pub struct GolExecute<'info> {
    pub bytecode: Account<'info, GolBytecode>,
}

#[account]
pub struct GolBytecode {
    pub handle: String,
    pub authority: Pubkey,
    pub finalized: bool,
    pub meta: Vec<u8>,
    pub content: Vec<u8>,
}
