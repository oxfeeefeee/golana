use crate::ffi::fmt2;
use borsh::de::BorshDeserialize;

pub fn run(bin: &Vec<u8>) {
    let bc = goscript_vm::Bytecode::try_from_slice(&bin).unwrap();

    let mut ffi = goscript_vm::FfiFactory::new();
    fmt2::Fmt2Ffi::register(&mut ffi);
    goscript_vm::run(&bc, &ffi, None);
}
