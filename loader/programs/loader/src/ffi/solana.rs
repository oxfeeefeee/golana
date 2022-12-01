use crate::goscript::Instruction;
use goscript_vm::types::*;
use goscript_vm::*;
use solana_program::{self, account_info::AccountInfo, pubkey::Pubkey};
use spl_token::{self, ID};

#[derive(Ffi)]
pub struct SolanaFfi;

#[ffi_impl]
impl SolanaFfi {
    fn ffi_get_ix(ctx: &mut FfiCtx) -> RuntimeResult<GosValue> {
        let ud = ctx.user_data.unwrap();
        let p = ud as *const Instruction;
        Ok(unsafe { p.as_ref() }.unwrap().get_ix(ctx))
    }

    fn ffi_find_program_address(
        ctx: &mut FfiCtx,
        seed: GosValue,
        program: GosValue,
    ) -> (GosValue, u8) {
        let program_id = Self::pub_key_from_ptr(ctx, &program)
            .expect("ffi_find_program_address: bad program id");
        let seed_str = seed.as_string().as_str();
        let (pk, bump) = Pubkey::find_program_address(&[seed_str.as_bytes()], &program_id);
        (Self::make_pub_key_ptr(ctx, pk), bump)
    }

    // fn ffi_token_set_authority(
    //     account_or_mint: GosValue,
    //     current_auth: GosValue,
    //     new_auth: GosValue,
    // ) -> RuntimeResult<()> {
    // }

    // fn ffi_token_transfer(
    //     from: GosValue,
    //     to: GosValue,
    //     anth: GosValue,
    //     amount: u64,
    //     signer_seeds: GosValue,
    // ) -> RuntimeResult<()> {
    // }

    // fn ffi_token_close_account(
    //     account: GosValue,
    //     dest: GosValue,
    //     auth: GosValue,
    //     signer_seeds: GosValue,
    // ) -> RuntimeResult<()> {
    // }

    fn pub_key_from_ptr(ctx: &FfiCtx, ptr: &GosValue) -> RuntimeResult<Pubkey> {
        let ptr_obj = ptr.as_non_nil_pointer()?;
        let pk = ptr_obj.deref(&ctx.stack, &ctx.vm_objs.packages)?;
        let slice: &[u8] = &FfiCtx::array_as_primitive_slice::<u8, u8>(&pk);
        Ok(Pubkey::new(slice))
    }

    #[inline]
    pub(crate) fn make_pub_key_ptr(ctx: &FfiCtx, key: Pubkey) -> GosValue {
        let pk = ctx.new_primitive_array(key.to_bytes().to_vec(), ValueType::Uint8);
        FfiCtx::new_pointer(pk)
    }
}
