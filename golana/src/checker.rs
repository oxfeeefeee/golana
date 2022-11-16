use crate::errors::*;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use goscript_vm::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountMeta {
    pub is_signer: bool,
    pub is_writable: bool,
    pub data: Option<types::Meta>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct IxMeta {
    pub name: String,
    pub accounts: Vec<AccountMeta>,
    pub args: Vec<types::Meta>,
}

impl IxMeta {
    fn new(name: &str, fields: &[types::FieldInfo], account: &types::Meta) -> Result<IxMeta> {
        let mut i = 0;
        let mut accounts: Vec<(AccountMeta, &str)> = vec![];
        // First, get all AccountInfo
        while i < fields.len() {
            if &fields[i].meta == account {
                accounts.push((
                    AccountMeta {
                        is_signer: false,
                        is_writable: false,
                        data: None,
                    },
                    &fields[i].name,
                ));
                i += 1;
            } else {
                break;
            }
        }

        // Then, the data declarations
        while i < fields.len() {
            let field = &fields[i];
            if field.meta.ptr_depth == 1 && field.name.ends_with("Data") {
                let mut found = false;
                for acc in accounts.iter_mut() {
                    if field.name.len() == acc.1.len() + "Data".len()
                        && field.name.starts_with(acc.1)
                    {
                        acc.0.data = Some(field.meta);
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(error!(GolError::WrongDataDeclare));
                }

                i += 1;
            } else {
                break;
            }
        }

        // Finally, arguments
        let mut args = vec![];
        while i < fields.len() {
            let meta = &fields[i].meta;
            if meta.is_type || meta.ptr_depth != 0 {
                return Err(error!(GolError::WrongArgType));
            }
            // todo: more checks
            args.push(meta.clone());
            i += 1;
        }

        Ok(IxMeta {
            name: name.to_owned(),
            accounts: accounts.into_iter().map(|(acc, _)| acc).collect(),
            args,
        })
    }
}

pub fn check(bc: &Bytecode) -> Result<Vec<IxMeta>> {
    let account_info_meta = get_account_info_meta(bc).ok_or(error!(GolError::MetaNotFound))?;

    let main_pkg = &bc.objects.packages[bc.main_pkg];
    let infos: Vec<(&String, &[types::FieldInfo])> = main_pkg
        .member_indices()
        .iter()
        .filter_map(|(name, index)| {
            (name.starts_with("Ix") && main_pkg.member(*index).typ() == types::ValueType::Metadata)
                .then(|| {
                    let member = main_pkg.member(*index);
                    let gmeta = member.as_metadata();
                    match &bc.objects.metas[gmeta.key].unwrap_named(&bc.objects.metas) {
                        types::MetadataType::Struct(f) => Some((name, f.infos())),
                        _ => None,
                    }
                })
                .flatten()
        })
        .collect();

    infos
        .into_iter()
        .map(|(name, fields)| IxMeta::new(name, fields, &account_info_meta))
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
        let xx = check(&bc);
        dbg!(&xx);
    }
}
