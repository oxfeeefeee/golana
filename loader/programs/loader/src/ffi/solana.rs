use crate::goscript::Instruction;
use anchor_lang::prelude::*;
use golana::*;
use goscript_vm::types::*;
use goscript_vm::*;
use solana_program::{self, account_info::AccountInfo, pubkey::Pubkey};
use spl_token::{self, instruction::AuthorityType};
use std::rc::Rc;

macro_rules! ref_seeds {
    ($seeds_vec: expr) => {{
        $seeds_vec.iter().map(|x| &x[..]).collect::<Vec<&[&[u8]]>>()
    }};
}

#[derive(UnsafePtr)]
pub struct Error(anyhow::Error);

#[derive(Ffi)]
pub struct SolanaFfi;

#[ffi_impl]
impl SolanaFfi {
    fn ffi_get_ix(ctx: &mut FfiCtx) -> GosValue {
        Self::get_instruction(ctx).get_ix(ctx)
    }

    fn ffi_commit_lamports(ctx: &mut FfiCtx, index: usize) {
        unimplemented!()
    }

    fn ffi_commit_data(ctx: &mut FfiCtx, index: usize) {
        unimplemented!()
    }

    fn ffi_commit_all(ctx: &mut FfiCtx, index: usize) {
        unimplemented!()
    }

    fn ffi_find_program_address(
        ctx: &mut FfiCtx,
        seed: GosValue,
        program: GosValue,
    ) -> (GosValue, u8) {
        let program_id =
            Self::get_pub_key(ctx, &program).expect("ffi_find_program_address: bad program id");
        let seed_str = seed.as_string().as_str();
        let (pk, bump) = Pubkey::find_program_address(&[seed_str.as_bytes()], &program_id);
        (Self::make_pub_key_ptr(ctx, pk), bump)
    }

    fn ffi_token_set_authority(
        ctx: &mut FfiCtx,
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
            let ix = spl_token::instruction::set_authority(
                &spl_token::ID,
                account_or_mint.key,
                spl_new_authority.as_ref(),
                Self::into_authority_type(auth_type)?,
                current_auth.key,
                &[], // TODO: Support multisig signers.
            )?;
            solana_program::program::invoke_signed(
                &ix,
                &[account_or_mint.clone(), current_auth.clone()],
                &ref_seeds!(Self::get_signers_seeds(&signer_seeds))[..],
            )
            .map_err(Into::into)
        })();
        Self::make_err(result)
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
            solana_program::program::invoke_signed(
                &ix,
                &[from.clone(), to.clone(), auth.clone()],
                &ref_seeds!(Self::get_signers_seeds(&signer_seeds))[..],
            )
            .map_err(Into::into)
        })();
        Self::make_err(result)
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
            solana_program::program::invoke_signed(
                &ix,
                &[account.clone(), dest.clone(), auth.clone()],
                &ref_seeds!(Self::get_signers_seeds(&signer_seeds))[..],
            )
            .map_err(Into::into)
        })();
        Self::make_err(result)
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
    pub(crate) fn make_err<T>(result: anyhow::Result<T>) -> GosValue {
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

    fn get_signers_seeds<'a>(seeds: &'a GosValue) -> Vec<[&'a [u8]; 2]> {
        if let Some((slice, _)) = seeds.as_gos_slice() {
            let data = slice.as_rust_slice();
            data.iter()
                .map(|x| {
                    let struct_ref = x.borrow();
                    let fields = struct_ref.as_struct().0.borrow_fields();
                    assert!(fields.len() == 2);
                    let seed: &[u8] = &fields[0].as_string().as_raw_slice();
                    let bump = fields[1].as_uint8();
                    unsafe {
                        [
                            std::slice::from_raw_parts(seed.as_ptr(), seed.len()),
                            std::slice::from_raw_parts(bump, 1),
                        ]
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }
}
