use crate::goscript::UserData;
use goscript_vm::value::*;
use goscript_vm::*;

#[derive(Ffi)]
pub struct Solana;

#[ffi_impl]
impl Solana {
    fn ffi_get_ix(ctx: &mut FfiCtx) -> RuntimeResult<GosValue> {
        Ok(ctx
            .user_data
            .unwrap()
            .as_any()
            .downcast_ref::<UserData>()
            .unwrap()
            .get_ix())
    }
}
