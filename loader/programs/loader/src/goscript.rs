use crate::ffi::{fmt2, solana};
use borsh::de::BorshDeserialize;
use golana::IxMeta;
use goscript_vm::types::{GosValue, ValueType};
use goscript_vm::*;
use solana_program::account_info::AccountInfo;
use std::any::Any;

pub fn run(bin: &Vec<u8>, meta_bin: &Vec<u8>, accounts: &[AccountInfo], args: Vec<u8>) {
    let bc = Bytecode::try_from_slice(bin).unwrap();
    let meta = Vec::<IxMeta>::try_from_slice(meta_bin).unwrap();
    let data = UserData::new(&meta, accounts, args);

    let mut ffi = goscript_vm::FfiFactory::with_user_data(&data);
    fmt2::Fmt2Ffi::register(&mut ffi);
    solana::Solana::register(&mut ffi);
    goscript_vm::run(&bc, &ffi, None);
}

pub(crate) struct UserData {
    gos_ix: GosValue,
}

impl UserData {
    fn new(meta: &[IxMeta], accounts: &[AccountInfo], args: Vec<u8>) -> UserData {
        UserData {
            gos_ix: UserData::deserialize_ix(meta, accounts, args),
        }
    }

    pub(crate) fn get_ix(&self) -> GosValue {
        self.gos_ix.clone()
    }

    fn deserialize_ix(meta: &[IxMeta], accounts: &[AccountInfo], args: Vec<u8>) -> GosValue {
        unimplemented!()
    }
}

impl<'a> UnsafePtr for UserData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
