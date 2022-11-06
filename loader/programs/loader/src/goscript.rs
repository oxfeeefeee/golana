use crate::ffi::{fmt2, solana};
use borsh::de::BorshDeserialize;
use goscript_vm::value::{GosValue, ValueType};
use goscript_vm::*;
use solana_program::account_info::AccountInfo;
use std::any::Any;

pub fn run(bin: &Vec<u8>, accounts: &[AccountInfo], args: Vec<u8>) {
    let bc = goscript_vm::Bytecode::try_from_slice(&bin).unwrap();
    let data = UserData::new(accounts, args);

    let mut ffi = goscript_vm::FfiFactory::with_user_data(&data);
    fmt2::Fmt2Ffi::register(&mut ffi);
    solana::Solana::register(&mut ffi);
    goscript_vm::run(&bc, &ffi, None);
}

pub(crate) struct UserData {
    gos_ix: GosValue,
}

impl UserData {
    fn new(accounts: &[AccountInfo], args: Vec<u8>) -> UserData {
        UserData {
            gos_ix: deserialize_ix(accounts, args),
        }
    }

    pub(crate) fn get_ix(&self) -> GosValue {
        self.gos_ix.clone()
    }
}

impl<'a> UnsafePtr for UserData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn deserialize_ix(accounts: &[AccountInfo], args: Vec<u8>) -> GosValue {
    unimplemented!()
}
