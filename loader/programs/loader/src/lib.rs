use anchor_lang::prelude::*;

declare_id!("55oqciWs2A8NRof7jSTrhi6HNpRhKWCMuYcEczGzkVy6");

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
    use std::vec;

    pub fn gol_initialize(ctx: Context<GolInitialize>, handle: String) -> Result<()> {
        let gol_pk = Pubkey::create_with_seed(ctx.accounts.authority.key, &handle, &ID).unwrap();
        let bytecode = &mut ctx.accounts.bytecode;
        require!(bytecode.key() == gol_pk, GolError::WrongHandle);

        bytecode.handle = handle;
        bytecode.authority = ctx.accounts.authority.key();
        bytecode.finalized = false;
        bytecode.content = vec![];
        Ok(())
    }

    pub fn gol_write(ctx: Context<GolWrite>, data: Vec<u8>) -> Result<()> {
        ctx.accounts.bytecode.content.extend(data.iter());
        Ok(())
    }

    pub fn gol_finalize(ctx: Context<GolFinalize>) -> Result<()> {
        ctx.accounts.bytecode.finalized = true;
        Ok(())
    }

    pub fn gol_execute(ctx: Context<GolExecute>, args: Vec<u8>) -> Result<()> {
        msg!(&ctx.remaining_accounts.len().to_string());
        crate::goscript::run(&ctx.accounts.bytecode.content, ctx.remaining_accounts, args);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(handle: String)]
pub struct GolInitialize<'info> {
    pub authority: Signer<'info>,
    #[account(zero)]
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
#[instruction(args: Vec<u8>)]
pub struct GolExecute<'info> {
    pub authority: Signer<'info>,
    #[account(has_one = authority)]
    pub bytecode: Account<'info, GolBytecode>,
}

#[account]
pub struct GolBytecode {
    pub handle: String,
    pub authority: Pubkey,
    pub finalized: bool,
    pub content: Vec<u8>,
}

#[error_code]
pub enum GolError {
    #[msg("Handle doesn't match against the public key")]
    WrongHandle,
}
