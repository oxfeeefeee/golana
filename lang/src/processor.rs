//! Program instruction processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let s = r#"
        package main

       // import "fmt2"

    func main() {
        // i := 1
        // i += 1
    	// fmt2.Println("hello golana", i)
        
    }
    "#;
    msg!("xxxxxxx");
    msg!(&std::mem::size_of::<usize>().to_string());

    crate::engine::run(s);

    Ok(())
}
