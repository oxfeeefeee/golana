use crate::config::*;
use crate::util::new_vm_program;
use anchor_client::solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use anchor_client::Program;
use anyhow::{anyhow, Result};
use solana_sdk;
use std::path::Path;

// deploy the project
pub fn deploy(config: &GolanaConfig, bc_path: &Path, force: bool) -> Result<()> {
    let program = new_vm_program(&config.provider)?;
    let seed = &config.project.name;
    let space = config.project.space;
    let rpc_client = program.rpc();

    let rent = rpc_client.get_minimum_balance_for_rent_exemption(space.try_into().unwrap())?;
    let bytecode_pk = Pubkey::create_with_seed(&program.payer(), seed, &program.id())?;

    // check if the account exists
    let account =
        rpc_client.get_account_with_commitment(&bytecode_pk, CommitmentConfig::processed())?;
    if account.value.is_some() {
        if !force {
            println!("Program already deployed, use -f to force re-deploy");
            return Err(anyhow!("Program already deployed"));
        }

        gol_clear(&program, &bytecode_pk, config.project.name.clone())?;
    } else {
        // create a new account
        let ix = solana_sdk::system_instruction::create_account_with_seed(
            &program.payer(),
            &bytecode_pk,
            &program.payer(),
            seed,
            rent,
            space,
            &program.id(),
        );
        program.request().instruction(ix).send()?;

        gol_initialize(&program, &bytecode_pk, config.project.name.clone())?;
    }

    let bytecode = std::fs::read(bc_path)?;
    gol_write(&program, &bytecode_pk, &bytecode)?;

    gol_finalize(&program, &bytecode_pk)?;

    Ok(())
}

/// Call Initialize instruction of Golana program
fn gol_initialize(program: &Program, bytecode_pk: &Pubkey, name: String) -> Result<()> {
    program
        .request()
        .instruction(
            solana_sdk::compute_budget::ComputeBudgetInstruction::request_heap_frame(256 * 1024),
        )
        .accounts(loader::accounts::GolInitialize {
            authority: program.payer(),
            bytecode: bytecode_pk.to_owned(),
        })
        .args(loader::instruction::GolInitialize { handle: name })
        .send()?;
    Ok(())
}

/// Call Clear instruction of Golana program
fn gol_clear(program: &Program, bytecode_pk: &Pubkey, name: String) -> Result<()> {
    program
        .request()
        .instruction(
            solana_sdk::compute_budget::ComputeBudgetInstruction::request_heap_frame(256 * 1024),
        )
        .accounts(loader::accounts::GolClear {
            authority: program.payer(),
            bytecode: bytecode_pk.to_owned(),
        })
        .args(loader::instruction::GolClear { handle: name })
        .send()?;
    Ok(())
}

/// Upload the bytecode to the account
fn gol_write(program: &Program, bytecode_pk: &Pubkey, bytecode: &[u8]) -> Result<()> {
    let chunk_size = 850;
    let mut offset = 0;
    while offset < bytecode.len() {
        println!("Writing chunk {}/{}", offset, bytecode.len());
        let chunk = &bytecode[offset..(offset + chunk_size).min(bytecode.len())];
        program
            .request()
            .options(CommitmentConfig::processed())
            .instruction(
                solana_sdk::compute_budget::ComputeBudgetInstruction::request_heap_frame(
                    256 * 1024,
                ),
            )
            .instruction(
                solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(
                    1400000,
                ),
            )
            .accounts(loader::accounts::GolWrite {
                authority: program.payer(),
                bytecode: bytecode_pk.to_owned(),
            })
            .args(loader::instruction::GolWrite {
                data: chunk.to_vec(),
            })
            .send()?;
        offset += chunk_size;
    }
    Ok(())
}

/// Call Finalize instruction of Golana program
fn gol_finalize(program: &Program, bytecode_pk: &Pubkey) -> Result<()> {
    program
        .request()
        .instruction(
            solana_sdk::compute_budget::ComputeBudgetInstruction::request_heap_frame(256 * 1024),
        )
        .instruction(
            solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(1400000),
        )
        .accounts(loader::accounts::GolFinalize {
            authority: program.payer(),
            bytecode: bytecode_pk.to_owned(),
        })
        .args(loader::instruction::GolFinalize {})
        .send()?;
    Ok(())
}
