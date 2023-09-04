use crate::goscript::Instruction;
use anchor_lang::error;
use go_vm::types::*;
use go_vm::*;
use golana::GolError;
use solana_program::program_option::COption;
use solana_program::{self, account_info::AccountInfo, pubkey::Pubkey};
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

    fn ffi_error_string(e: GosValue) -> RuntimeResult<String> {
        let rust_err = e.as_non_nil_unsafe_ptr()?.downcast_ref::<Error>()?;
        Ok(rust_err.0.to_string())
    }

    fn ffi_log_compute_unit() {
        solana_program::log::sol_log_compute_units();
    }

    fn ffi_find_program_address(ctx: &FfiCtx, seed: GosValue, program: GosValue) -> (GosValue, u8) {
        let program_id =
            Self::get_pub_key(ctx, &program).expect("ffi_find_program_address: bad program id");
        let hashed = Self::get_seed_hash(seed.as_string().as_str().as_bytes(), &program_id);
        let (pk, bump) = Pubkey::find_program_address(&[&hashed[..]], &crate::ID);
        (Self::make_pub_key_ptr(ctx, pk), bump)
    }

    fn ffi_account_create(
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
        Self::unwrap_empty_result(result)
    }

    fn ffi_account_key(ctx: &FfiCtx, index: usize) -> GosValue {
        let inst = Self::get_instruction(ctx);
        Self::make_pub_key_ptr(ctx, *inst.accounts[index].key)
    }

    fn ffi_account_lamports(ctx: &FfiCtx, index: usize) -> u64 {
        let inst = Self::get_instruction(ctx);
        **inst.accounts[index].lamports.borrow()
    }

    fn ffi_set_account_lamports(ctx: &FfiCtx, index: usize, lamports: u64) {
        let inst = Self::get_instruction(ctx);
        **inst.accounts[index].lamports.borrow_mut() = lamports;
    }

    fn ffi_account_owner(ctx: &FfiCtx, index: usize) -> GosValue {
        let inst = Self::get_instruction(ctx);
        Self::make_pub_key_ptr(ctx, *inst.accounts[index].owner)
    }

    fn ffi_account_executable(ctx: &FfiCtx, index: usize) -> bool {
        let inst = Self::get_instruction(ctx);
        inst.accounts[index].executable
    }

    fn ffi_account_rent_epoch(ctx: &FfiCtx, index: usize) -> u64 {
        let inst = Self::get_instruction(ctx);
        inst.accounts[index].rent_epoch
    }

    fn ffi_account_data(ctx: &FfiCtx, index: usize) -> GosValue {
        let inst = Self::get_instruction(ctx);
        let account = &inst.accounts[index];
        let account_meta = &inst.ix_meta.accounts[index];
        let result = || -> anyhow::Result<GosValue> {
            if let Some(data_meta) = &account_meta.data_meta {
                let mut buf: &[u8] = &account.data.borrow();
                GosValue::deserialize_wo_type(data_meta, &ctx.vm_objs.metas, &mut buf)
                    .map(|val| {
                        ctx.new_empty_interface(FfiCtx::new_pointer(val), data_meta.ptr_to())
                    })
                    .map_err(Into::into)
            } else {
                Err(error!(GolError::MethodNotFound)).map_err(Into::into)
            }
        }();
        result.unwrap()
    }

    fn ffi_account_save_data(ctx: &FfiCtx, index: usize, data_iface: GosValue) {
        let inst = Self::get_instruction(ctx);
        let account = &inst.accounts[index];
        let account_meta = &inst.ix_meta.accounts[index];
        let result = || -> anyhow::Result<()> {
            if let Some(_) = &account_meta.data_meta {
                let mut buf: &mut [u8] = &mut account.data.borrow_mut();
                let data_ptr = data_iface
                    .as_non_nil_interface()?
                    .underlying_value()
                    .unwrap();
                let data_obj = ctx.deref_pointer(&data_ptr).unwrap();
                GosValue::serialize_wo_type(&data_obj, &mut buf).map_err(Into::into)
            } else {
                Err(error!(GolError::MethodNotFound)).map_err(Into::into)
            }
        }();
        result.unwrap();
    }

    pub(crate) fn invoke_signed(
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

    #[inline]
    pub(crate) fn make_pub_key_ptr(ctx: &FfiCtx, key: Pubkey) -> GosValue {
        let pk = ctx.new_primitive_array(key.to_bytes().to_vec(), ValueType::Uint8);
        FfiCtx::new_pointer(pk)
    }

    #[inline]
    pub(crate) fn make_pub_key_nilable_ptr(ctx: &FfiCtx, key: COption<Pubkey>) -> GosValue {
        match key {
            COption::Some(key) => Self::make_pub_key_ptr(ctx, key),
            COption::None => FfiCtx::new_nil(ValueType::Pointer),
        }
    }

    #[inline]
    pub(crate) fn unwrap_result(result: anyhow::Result<GosValue>) -> (GosValue, GosValue) {
        match result {
            Ok(v) => (v, FfiCtx::new_nil(ValueType::UnsafePtr)),
            Err(e) => (
                FfiCtx::new_nil(ValueType::Void),
                FfiCtx::new_unsafe_ptr(Rc::new(Error(e))),
            ),
        }
    }

    #[inline]
    pub(crate) fn unwrap_empty_result(result: anyhow::Result<()>) -> GosValue {
        match result {
            Ok(_) => FfiCtx::new_nil(ValueType::UnsafePtr),
            Err(e) => FfiCtx::new_unsafe_ptr(Rc::new(Error(e))),
        }
    }

    pub(crate) fn get_instruction<'a, 'info>(ctx: &'a FfiCtx) -> &'a Instruction<'a, 'info> {
        let ud = ctx.user_data.unwrap();
        let p = ud as *const Instruction;
        unsafe { p.as_ref() }.unwrap()
    }

    pub(crate) fn get_pub_key(ctx: &FfiCtx, ptr: &GosValue) -> RuntimeResult<Pubkey> {
        let ptr_obj = ptr.as_non_nil_pointer()?;
        let pk = ptr_obj.deref(&ctx.stack, &ctx.vm_objs.packages)?;
        let slice: &[u8] = &FfiCtx::array_as_primitive_slice::<u8, u8>(&pk);
        Ok(Pubkey::from(<[u8; 32]>::try_from(slice).unwrap()))
    }

    pub(crate) fn get_signers_seed_buf(seeds: &GosValue, program_id: &Pubkey) -> Vec<u8> {
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

    pub(crate) fn get_seed_hash(
        seed: &[u8],
        program_id: &Pubkey,
    ) -> [u8; solana_program::hash::HASH_BYTES] {
        let mut full_seed = program_id.to_bytes().to_vec();
        full_seed.append(&mut seed.to_owned());
        solana_program::hash::hash(&full_seed).to_bytes()
    }
}
