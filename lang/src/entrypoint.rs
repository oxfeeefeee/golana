//! Program entrypoint

#![cfg(not(feature = "no-entrypoint"))]

use smalloc::Smalloc;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::{ProgramResult, HEAP_LENGTH, HEAP_START_ADDRESS},
    pubkey::Pubkey,
};

#[cfg(target_os = "solana")]
#[global_allocator]
static ALLOC: Smalloc = Smalloc {
    start: HEAP_START_ADDRESS as usize,
    length: HEAP_LENGTH as usize,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::processor::process_instruction(program_id, accounts, instruction_data)
}
