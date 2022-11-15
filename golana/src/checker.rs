use std::borrow::Borrow;
use std::mem;

use crate::errors::*;
use anchor_lang::prelude::*;
use borsh::de::BorshDeserialize;
use goscript_vm::types::{GosValue, Meta as GosMeta, ValueType};
use goscript_vm::*;
use solana_program::account_info::AccountInfo;

#[derive(Debug)]
pub struct AccountMeta {
    pub is_signer: bool,
    pub is_writable: bool,
    pub data: Option<GosMeta>,
}

#[derive(Debug)]
pub struct IxMeta {
    pub name: String,
    pub accounts: Vec<AccountMeta>,
    pub args: Vec<GosMeta>,
}

impl IxMeta {
    fn new(name: &str, gmeta: &[types::FieldInfo]) -> IxMeta {
        // First, get all AccountInfo
        let mut accounts: Vec<AccountMeta> = vec![];

        unimplemented!()
    }
}

pub fn check(bc: &Bytecode) -> Vec<IxMeta> {
    let main_pkg = &bc.objects.packages[bc.main_pkg];
    main_pkg
        .member_indices()
        .iter()
        .filter_map(|(name, index)| {
            (name.starts_with("Ix") && main_pkg.member(*index).typ() == ValueType::Metadata)
                .then(|| {
                    let member = main_pkg.member(*index);
                    let gmeta = member.as_metadata();
                    match &bc.objects.metas[gmeta.key].unwrap_named(&bc.objects.metas) {
                        types::MetadataType::Struct(f) => Some(IxMeta::new(name, f.infos())),
                        _ => None,
                    }
                })
                .flatten()
        })
        .collect()
}

fn get_account_info_meta(bc: &Bytecode) -> Option<types::Meta> {
    let key = bc
        .objects
        .packages
        .iter()
        .enumerate()
        .find(|(_, pkg)| pkg.name() == "solana")
        .map(|(i, _)| i.into())?;
    let pkg = &bc.objects.packages[key];
    let member = pkg.member(*pkg.member_index("AccountInfo")?);
    match member.typ() {
        types::ValueType::Metadata => Some(member.as_metadata().clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::path::Path;

    fn read_bytecode(full_name: &Path) -> Bytecode {
        let mut f = std::fs::OpenOptions::new()
            .read(true)
            .open(full_name)
            .expect("no file found");
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer).expect("read file error");
        Bytecode::try_from_slice(&buffer).expect("deserialize error")
    }

    #[test]
    fn it_works() {
        let bc = read_bytecode(Path::new("../examples/escrow/target/escrow.gosb"));
        let acc_meta = get_account_info_meta(&bc);
        dbg!(acc_meta);

        let metas = check(&bc);
        dbg!(&metas);
    }
}
