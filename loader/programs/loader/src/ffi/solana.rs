use crate::goscript::Instruction;
use anchor_lang::prelude::*;
use golana::*;
use goscript_vm::types::*;
use goscript_vm::*;
use solana_program::{self, account_info::AccountInfo, program_pack::Pack, pubkey::Pubkey};
use spl_token::{self, instruction::AuthorityType};
use std::rc::Rc;

#[derive(UnsafePtr)]
pub struct Error(anyhow::Error);

#[derive(Ffi)]
pub struct SolanaFfi;

#[ffi_impl]
impl SolanaFfi {
    fn ffi_get_ix(ctx: &FfiCtx) -> GosValue {
        Self::get_instruction(ctx).get_ix(ctx)
    }

    fn ffi_get_id(ctx: &FfiCtx) -> GosValue {
        let ix = Self::get_instruction(ctx);
        Self::make_pub_key_ptr(ctx, ix.gos_program_id.clone())
    }

    fn ffi_commit_lamports(ctx: &FfiCtx, index: usize) {
        Self::get_instruction(ctx)
            .write_back_data(ctx, index..index + 1, true, false)
            .unwrap();
    }

    fn ffi_commit_data(ctx: &FfiCtx, index: usize) {
        Self::get_instruction(ctx)
            .write_back_data(ctx, index..index + 1, false, true)
            .expect("commit data error!");
    }

    fn ffi_commit_lamports_and_data(ctx: &FfiCtx, index: usize) {
        Self::get_instruction(ctx)
            .write_back_data(ctx, index..index + 1, true, true)
            .expect("commit lamports and data error!");
    }

    fn ffi_commit_everything(ctx: &FfiCtx) {
        let ix = Self::get_instruction(ctx);
        ix.write_back_data(ctx, 0..ix.accounts.len(), true, true)
            .expect("commit everything error!");
    }

    fn ffi_error_string(e: GosValue) -> RuntimeResult<String> {
        let rust_err = e.as_non_nil_unsafe_ptr()?.downcast_ref::<Error>()?;
        Ok(rust_err.0.to_string())
    }

    fn ffi_find_program_address(ctx: &FfiCtx, seed: GosValue, program: GosValue) -> (GosValue, u8) {
        let program_id =
            Self::get_pub_key(ctx, &program).expect("ffi_find_program_address: bad program id");
        let hashed = Self::get_seed_hash(seed.as_string().as_str().as_bytes(), &program_id);
        let (pk, bump) = Pubkey::find_program_address(&[&hashed[..]], &crate::ID);
        (Self::make_pub_key_ptr(ctx, pk), bump)
    }

    fn ffi_create_account(
        ctx: &FfiCtx,
        from_index: usize,
        to_index: usize,
        owner: GosValue,
        lamports: u64,
        space: u64,
        signer_seeds: GosValue,
    ) -> GosValue {
        let inst = Self::get_instruction(ctx);
        let from = inst.accounts[from_index].clone();
        let to = inst.accounts[to_index].clone();
        let owner_pk = Self::get_pub_key(ctx, &owner).expect("ffi_create_account: bad owner");
        let ix = solana_program::system_instruction::create_account(
            from.key, to.key, lamports, space, &owner_pk,
        );
        let result = Self::invoke_signed(&ix, &[from, to], signer_seeds, inst.gos_program_id);
        Self::make_err_unsafe_ptr(result)
    }

    fn ffi_token_init_account(
        ctx: &FfiCtx,
        account_index: usize,
        mint_index: usize,
        auth_index: usize,
        rent_index: usize,
        signer_seeds: GosValue,
    ) -> GosValue {
        let inst = Self::get_instruction(ctx);
        let account = inst.accounts[account_index].clone();
        let mint = inst.accounts[mint_index].clone();
        let auth = inst.accounts[auth_index].clone();
        let rent = inst.accounts[rent_index].clone();
        let result: anyhow::Result<()> = (move || {
            let ix = spl_token::instruction::initialize_account(
                &spl_token::ID,
                account.key,
                mint.key,
                auth.key,
            )?;
            Self::invoke_signed(
                &ix,
                &[account, mint, auth, rent],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        Self::make_err_unsafe_ptr(result)
    }

    fn ffi_token_create_and_init_account(
        ctx: &FfiCtx,
        from_index: usize,
        to_index: usize,
        token_program: GosValue,
        mint_index: usize,
        auth_index: usize,
        rent_index: usize,
        signer_seeds: GosValue,
    ) -> GosValue {
        let inst = Self::get_instruction(ctx);
        let from = inst.accounts[from_index].clone();
        let to = inst.accounts[to_index].clone();
        let owner = Self::get_pub_key(ctx, &token_program)
            .expect("ffi_token_create_and_init_account: bad token program id");
        let mint = inst.accounts[mint_index].clone();
        let auth = inst.accounts[auth_index].clone();
        let rent = inst.accounts[rent_index].clone();
        let result: anyhow::Result<()> = (move || {
            let len = spl_token::state::Account::LEN;
            let space = len as u64;
            let sol_rent = Rent::get()?;
            let lamports = sol_rent.minimum_balance(len);
            let ix = solana_program::system_instruction::create_account(
                from.key, to.key, lamports, space, &owner,
            );
            Self::invoke_signed(
                &ix,
                &[from, to.clone()],
                signer_seeds.clone(),
                inst.gos_program_id,
            )?;

            let ix = spl_token::instruction::initialize_account(
                &spl_token::ID,
                to.key,
                mint.key,
                auth.key,
            )?;
            Self::invoke_signed(
                &ix,
                &[to, mint, auth, rent],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        Self::make_err_unsafe_ptr(result)
    }

    fn ffi_token_close_account(
        ctx: &FfiCtx,
        account_index: usize,
        dest_index: usize,
        auth_index: usize,
        signer_seeds: GosValue,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = Self::get_instruction(ctx);
            let account = &inst.accounts[account_index];
            let dest = &inst.accounts[dest_index];
            let auth = &inst.accounts[auth_index];
            let ix = spl_token::instruction::close_account(
                &spl_token::ID,
                account.key,
                dest.key,
                auth.key,
                &[], // TODO: support multisig
            )?;
            Self::invoke_signed(
                &ix,
                &[account.clone(), dest.clone(), auth.clone()],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        Self::make_err_unsafe_ptr(result)
    }

    fn ffi_token_set_authority(
        ctx: &FfiCtx,
        account_or_mint_index: usize,
        current_auth_index: usize,
        new_auth_key: GosValue,
        auth_type: u8,
        signer_seeds: GosValue,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = Self::get_instruction(ctx);
            let account_or_mint = &inst.accounts[account_or_mint_index];
            let current_auth = &inst.accounts[current_auth_index];
            let mut spl_new_authority: Option<Pubkey> = None;
            if !new_auth_key.is_nil() {
                spl_new_authority = Some(Self::get_pub_key(ctx, &new_auth_key)?);
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
            Self::invoke_signed(
                &ix,
                &[account_or_mint.clone(), current_auth.clone()],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        Self::make_err_unsafe_ptr(result)
    }

    fn ffi_token_transfer(
        ctx: &FfiCtx,
        from_index: usize,
        to_index: usize,
        auth_index: usize,
        amount: u64,
        signer_seeds: GosValue,
    ) -> GosValue {
        let result: anyhow::Result<()> = (move || {
            let inst = Self::get_instruction(ctx);
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
            Self::invoke_signed(
                &ix,
                &[from.clone(), to.clone(), auth.clone()],
                signer_seeds,
                inst.gos_program_id,
            )
        })();
        Self::make_err_unsafe_ptr(result)
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

    fn invoke_signed(
        instruction: &solana_program::instruction::Instruction,
        account_infos: &[AccountInfo],
        signer_seeds: GosValue,
        program_id: &Pubkey,
    ) -> anyhow::Result<()> {
        if !signer_seeds.is_nil() {
            let buf = Self::get_signers_seed_buf(&signer_seeds, program_id);
            let mut s = &buf[..];
            let mut groups: Vec<[&[u8]; 2]> = vec![];
            let hb = solana_program::hash::HASH_BYTES;
            while s.len() > 0 {
                groups.push([&s[0..hb], &s[hb..hb + 1]]);
                s = &s[hb + 1..];
            }
            let refs = groups.iter().map(|x| &x[..]).collect::<Vec<&[&[u8]]>>();
            solana_program::program::invoke_signed(instruction, account_infos, &refs[..])
        } else {
            solana_program::program::invoke_signed(instruction, account_infos, &vec![])
        }
        .map_err(Into::into)
    }

    pub(crate) fn make_account_info_ptr(ctx: &FfiCtx, ai: &AccountInfo, index: usize) -> GosValue {
        let key = Self::make_pub_key_ptr(ctx, *ai.key);
        let lamports: GosValue = (**ai.lamports.borrow()).into();
        let owner = Self::make_pub_key_ptr(ctx, *ai.owner);
        let executable: GosValue = ai.executable.into();
        let rent_epoch: GosValue = ai.rent_epoch.into();
        let index_gos: GosValue = index.into();
        let account_info = ctx.new_struct(vec![
            key, lamports, owner, executable, rent_epoch, index_gos,
        ]);
        FfiCtx::new_pointer(account_info)
    }

    #[inline]
    pub(crate) fn make_pub_key_ptr(ctx: &FfiCtx, key: Pubkey) -> GosValue {
        let pk = ctx.new_primitive_array(key.to_bytes().to_vec(), ValueType::Uint8);
        FfiCtx::new_pointer(pk)
    }

    #[inline]
    pub(crate) fn make_err_unsafe_ptr<T>(result: anyhow::Result<T>) -> GosValue {
        match result {
            Ok(_) => FfiCtx::new_nil(ValueType::UnsafePtr),
            Err(e) => FfiCtx::new_unsafe_ptr(Rc::new(Error(e))),
        }
    }

    fn get_instruction<'a, 'info>(ctx: &'a FfiCtx) -> &'a Instruction<'a, 'info> {
        let ud = ctx.user_data.unwrap();
        let p = ud as *const Instruction;
        unsafe { p.as_ref() }.unwrap()
    }

    fn get_pub_key(ctx: &FfiCtx, ptr: &GosValue) -> RuntimeResult<Pubkey> {
        let ptr_obj = ptr.as_non_nil_pointer()?;
        let pk = ptr_obj.deref(&ctx.stack, &ctx.vm_objs.packages)?;
        let slice: &[u8] = &FfiCtx::array_as_primitive_slice::<u8, u8>(&pk);
        Ok(Pubkey::new(slice))
    }

    fn get_signers_seed_buf(seeds: &GosValue, program_id: &Pubkey) -> Vec<u8> {
        if let Some((slice, _)) = seeds.as_gos_slice() {
            let data = slice.as_rust_slice();
            data.iter().fold(vec![], |mut acc, x| {
                let struct_ref = x.borrow();
                let fields = struct_ref.as_struct().0.borrow_fields();
                assert!(fields.len() == 2);
                let seed: &[u8] = &fields[0].as_string().as_raw_slice();
                let mut hashed: Vec<u8> = Self::get_seed_hash(seed, program_id).try_into().unwrap();
                let bump = fields[1].as_uint8();
                acc.append(&mut hashed);
                acc.push(*bump);
                acc
            })
        } else {
            vec![]
        }
    }

    fn get_seed_hash(seed: &[u8], program_id: &Pubkey) -> [u8; solana_program::hash::HASH_BYTES] {
        let mut full_seed = program_id.to_bytes().to_vec();
        full_seed.append(&mut seed.to_owned());
        solana_program::hash::hash(&full_seed).to_bytes()
    }
}
