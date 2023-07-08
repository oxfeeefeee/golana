use anchor_lang::prelude::*;
use go_vm::types::*;
use go_vm::*;
use golana::*;
use solana_program::{self, program_pack::Pack, pubkey::Pubkey};
use spl_associated_token_account::{
    self, instruction::create_associated_token_account,
    instruction::create_associated_token_account_idempotent,
};
use spl_token::{self, instruction::AuthorityType, state::Mint};

use super::solana::SolanaFfi;

#[derive(Ffi)]
pub struct TokenFfi;

#[ffi_impl]
impl TokenFfi {
    fn ffi_unpack_mint(ctx: &FfiCtx, account_index: usize) -> (GosValue, GosValue) {
        let inst = SolanaFfi::get_instruction(ctx);
        let account = &inst.accounts[account_index];
        let mint = Mint::unpack(&account.data.borrow());
        let result: anyhow::Result<GosValue> = (move || {
            let mint = mint?;
            let mint_authority = SolanaFfi::make_pub_key_nilable_ptr(ctx, mint.mint_authority);
            let supply = GosValue::from(mint.supply);
            let decimals = GosValue::from(mint.decimals);
            let is_initialized = GosValue::from(mint.is_initialized);
            let freeze_authority = SolanaFfi::make_pub_key_nilable_ptr(ctx, mint.freeze_authority);
            let mint = ctx.new_struct(vec![
                mint_authority,
                supply,
                decimals,
                is_initialized,
                freeze_authority,
            ]);
            Ok(mint)
        })();
        SolanaFfi::unwrap_result(result)
    }

    fn ffi_create_and_init_account(
        ctx: &FfiCtx,
        from_index: usize,
        to_index: usize,
        mint_index: usize,
        owner: GosValue,
        signer_seeds: GosValue,
    ) -> GosValue {
        let inst = SolanaFfi::get_instruction(ctx);
        let from = inst.accounts[from_index].clone();
        let to = inst.accounts[to_index].clone();
        let mint = inst.accounts[mint_index].clone();
        let owner =
            SolanaFfi::get_pub_key(ctx, &owner).expect("ffi_create_and_init_account: bad owner");
        let result: anyhow::Result<()> = (move || {
            let len = spl_token::state::Account::LEN;
            let space = len as u64;
            let sol_rent = Rent::get()?;
            let lamports = sol_rent.minimum_balance(len);
            let ix = solana_program::system_instruction::create_account(
                from.key,
                to.key,
                lamports,
                space,
                &spl_token::ID,
            );
            SolanaFfi::invoke_signed(
                &ix,
                &[from, to.clone()],
                signer_seeds.clone(),
                inst.gos_program_id,
            )?;

            let ix = spl_token::instruction::initialize_account3(
                &spl_token::ID,
                to.key,
                mint.key,
                &owner,
            )?;
            SolanaFfi::invoke_signed(&ix, &[to, mint], signer_seeds, inst.gos_program_id)
        })();
        SolanaFfi::unwrap_empty_result(result)
    }

    fn ffi_close_account(
        ctx: &FfiCtx,
        account_index: usize,
        dest_index: usize,
        owner_index: usize,
        signer_seeds: GosValue,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = SolanaFfi::get_instruction(ctx);
            let account = &inst.accounts[account_index];
            let dest = &inst.accounts[dest_index];
            let owner = &inst.accounts[owner_index];
            let ix = spl_token::instruction::close_account(
                &spl_token::ID,
                account.key,
                dest.key,
                owner.key,
                &[], // TODO: support multisig
            )?;
            SolanaFfi::invoke_signed(
                &ix,
                &[account.clone(), dest.clone(), owner.clone()],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        SolanaFfi::unwrap_empty_result(result)
    }

    fn ffi_set_authority(
        ctx: &FfiCtx,
        account_or_mint_index: usize,
        current_auth_index: usize,
        new_auth_key: GosValue,
        auth_type: u8,
        signer_seeds: GosValue,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = SolanaFfi::get_instruction(ctx);
            let account_or_mint = &inst.accounts[account_or_mint_index];
            let current_auth = &inst.accounts[current_auth_index];
            let mut spl_new_authority: Option<Pubkey> = None;
            if !new_auth_key.is_nil() {
                spl_new_authority = Some(SolanaFfi::get_pub_key(ctx, &new_auth_key)?);
            }
            msg!(&spl_new_authority.unwrap().to_string());
            let ix = spl_token::instruction::set_authority(
                &spl_token::ID,
                account_or_mint.key,
                spl_new_authority.as_ref(),
                Self::into_authority_type(auth_type)?,
                current_auth.key,
                &[], // TODO: Support multisig signers.
            )?;
            SolanaFfi::invoke_signed(
                &ix,
                &[account_or_mint.clone(), current_auth.clone()],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        SolanaFfi::unwrap_empty_result(result)
    }

    fn ffi_transfer(
        ctx: &FfiCtx,
        from_index: usize,
        to_index: usize,
        auth_index: usize,
        amount: u64,
        signer_seeds: GosValue,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = SolanaFfi::get_instruction(ctx);
            let from = &inst.accounts[from_index];
            let to = &inst.accounts[to_index];
            let auth = &inst.accounts[auth_index];
            let ix = spl_token::instruction::transfer(
                &spl_token::ID,
                from.key,
                to.key,
                auth.key,
                &[],
                amount,
            )?;
            SolanaFfi::invoke_signed(
                &ix,
                &[from.clone(), to.clone(), auth.clone()],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        SolanaFfi::unwrap_empty_result(result)
    }

    fn ffi_mint_to(
        ctx: &FfiCtx,
        mint_index: usize,
        dest_index: usize,
        auth_index: usize,
        amount: u64,
        signer_seeds: GosValue,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = SolanaFfi::get_instruction(ctx);
            let mint = &inst.accounts[mint_index];
            let dest = &inst.accounts[dest_index];
            let auth = &inst.accounts[auth_index];
            let ix = spl_token::instruction::mint_to(
                &spl_token::ID,
                mint.key,
                dest.key,
                auth.key,
                &[],
                amount,
            )?;
            SolanaFfi::invoke_signed(
                &ix,
                &[mint.clone(), dest.clone(), auth.clone()],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        SolanaFfi::unwrap_empty_result(result)
    }

    fn ffi_burn(
        ctx: &FfiCtx,
        account_index: usize,
        mint_index: usize,
        auth_index: usize,
        amount: u64,
        signer_seeds: GosValue,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = SolanaFfi::get_instruction(ctx);
            let account = &inst.accounts[account_index];
            let mint = &inst.accounts[mint_index];
            let auth = &inst.accounts[auth_index];
            let ix = spl_token::instruction::burn(
                &spl_token::ID,
                account.key,
                mint.key,
                auth.key,
                &[],
                amount,
            )?;
            SolanaFfi::invoke_signed(
                &ix,
                &[account.clone(), mint.clone(), auth.clone()],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        SolanaFfi::unwrap_empty_result(result)
    }

    fn ffi_create_associated_account(
        ctx: &FfiCtx,
        payer_index: usize,
        dest_index: usize,
        owner_index: usize,
        mint_index: usize,
        sys_index: usize,
        spl_index: usize,
        idempotent: bool,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = SolanaFfi::get_instruction(ctx);
            let mint = &inst.accounts[mint_index];
            let owner = &inst.accounts[owner_index];
            let payer = &inst.accounts[payer_index];
            let dest = &inst.accounts[dest_index];
            let ix = if idempotent {
                create_associated_token_account_idempotent(
                    payer.key,
                    owner.key,
                    mint.key,
                    &spl_token::ID,
                )
            } else {
                create_associated_token_account(payer.key, owner.key, mint.key, &spl_token::ID)
            };
            SolanaFfi::invoke_signed(
                &ix,
                &[
                    payer.clone(),
                    dest.clone(),
                    owner.clone(),
                    mint.clone(),
                    inst.accounts[sys_index].clone(),
                    inst.accounts[spl_index].clone(),
                ],
                FfiCtx::new_nil(ValueType::Void),
                inst.gos_program_id,
            )
        })();
        SolanaFfi::unwrap_empty_result(result)
    }

    fn into_authority_type(val: u8) -> Result<AuthorityType> {
        match val {
            0 => Ok(AuthorityType::MintTokens),
            1 => Ok(AuthorityType::FreezeAccount),
            2 => Ok(AuthorityType::AccountOwner),
            3 => Ok(AuthorityType::CloseAccount),
            _ => Err(error!(GolError::RtCheckAccountCount)),
        }
    }
}
