use goscript_engine;
use goscript_engine::ffi::*;
use solana_program::msg;

#[derive(Ffi)]
pub struct Fmt2Ffi;

#[ffi_impl]
impl Fmt2Ffi {
    fn ffi_println(args: GosValue) -> RuntimeResult<()> {
        let vec = args
            .as_non_nil_slice::<GosElem>()?
            .0
            .get_vec(ValueType::Interface);
        let strs: Vec<String> = vec
            .iter()
            .map(|x| {
                let s = if x.is_nil() {
                    "<nil>".to_owned()
                } else {
                    let underlying = x.iface_underlying()?;
                    match underlying {
                        Some(v) => v.to_string(),
                        None => "<ffi>".to_owned(),
                    }
                };
                Ok(s)
            })
            .map(|x: RuntimeResult<String>| x.unwrap())
            .collect();
        msg!("{}", strs.join(", "));
        Ok(())
    }

    fn ffi_printf(_args: GosValue) {
        unimplemented!();
    }
}
