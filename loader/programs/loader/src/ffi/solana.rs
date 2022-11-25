use crate::goscript::Instruction;
use goscript_vm::types::*;
use goscript_vm::*;

#[derive(Ffi)]
pub struct SolanaFfi;

#[ffi_impl]
impl SolanaFfi {
    fn ffi_get_ix(ctx: &mut FfiCtx) -> RuntimeResult<GosValue> {
        let ud = ctx.user_data.unwrap();
        let p = ud as *const Instruction;
        Ok(unsafe { p.as_ref() }.unwrap().get_ix(ctx))
    }
}
