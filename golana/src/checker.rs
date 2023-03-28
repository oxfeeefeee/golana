use crate::errors::*;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use goscript_vm::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub enum AccessMode {
    None,
    Initialize(usize),
    ReadOnly(usize),
    Mutable(usize),
}

impl AccessMode {
    pub fn try_with_name(name: &str, index: usize) -> Option<AccessMode> {
        match name {
            "_data" => Some(Self::ReadOnly(index)),
            "_dataInit" => Some(Self::Initialize(index)),
            "_dataMut" => Some(Self::Mutable(index)),
            _ => None,
        }
    }

    pub fn get_data_index(&self) -> Option<usize> {
        match self {
            Self::Initialize(i) | Self::ReadOnly(i) | Self::Mutable(i) => Some(*i),
            _ => None,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccMeta {
    pub name: String,
    pub is_signer: bool,
    pub is_mut: bool,
    pub access_mode: AccessMode,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct IxMeta {
    pub name: String,
    pub gos_meta: types::Meta,
    pub process_method: types::FunctionKey,
    pub process_method_index: usize,
    pub accounts: Vec<AccMeta>,
    pub accounts_data: Vec<(usize, types::Meta)>,
    pub args: Vec<(String, types::Meta)>,
}

impl IxMeta {
    fn new(
        name: &str,
        gos_meta: types::Meta,
        account: &types::Meta,
        metas: &types::MetadataObjs,
    ) -> Result<IxMeta> {
        let (methods, inner_meta) = metas[gos_meta.key].as_named();
        let process_method_index = methods
            .mapping
            .iter()
            .find_map(|(name, index)| (name == "Process").then_some(*index as usize))
            .ok_or(error!(GolError::MethodNotFound))?;
        let method_desc = (&*methods.members[process_method_index]).borrow();
        let process_method = method_desc
            .pointer_recv
            .then_some(method_desc.func.unwrap())
            .ok_or(error!(GolError::NonPointerReceiver))?;

        // Build struct fields
        let fields = metas[inner_meta.key].as_struct().infos();
        let mut i = 0;
        let mut accounts: Vec<(AccMeta, &str)> = vec![];

        // First, get all AccountInfo
        while i < fields.len() {
            let meta = &fields[i].meta;
            let name = &fields[i].name;
            let tags = &fields[i].lookup_tag("golana");
            let (is_signer, is_mut) = Self::is_signer_or_mut(tags);
            if meta.key == account.key {
                if meta.ptr_depth != 1 {
                    return Err(error!(GolError::NonPointerAccountInfo));
                }
                accounts.push((
                    AccMeta {
                        // todo: proper parsing
                        name: name.to_owned(),
                        is_signer,
                        is_mut,
                        access_mode: AccessMode::None,
                    },
                    &fields[i].name,
                ));
                i += 1;
            } else {
                break;
            }
        }

        // Then, the data declarations
        let mut accounts_data = vec![];
        while i < fields.len() {
            let field = &fields[i];
            if field.meta.ptr_depth == 1 {
                let mut found = false;
                for (index, acc) in accounts.iter_mut().enumerate() {
                    if field.name.starts_with(acc.1) {
                        let post_fix = &field.name[acc.1.len()..];
                        if post_fix != "_data" {
                            return Err(error!(GolError::AccountNamePrefixReserved));
                        }
                        let data_index = accounts_data.len();
                        let mode =
                            Self::get_data_access_mode(&field.lookup_tag("golana"), data_index)?;
                        if acc.0.access_mode != AccessMode::None {
                            return Err(error!(GolError::DuplicatedDataDeclare));
                        }
                        acc.0.access_mode = mode;
                        if field.meta.ptr_depth != 1 {
                            return Err(error!(GolError::NonPointerDataDeclare));
                        }
                        accounts_data.push((index, field.meta.unptr_to()));
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(error!(GolError::BadDataDeclare));
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
            args.push((fields[i].name.clone(), meta.clone()));
            i += 1;
        }

        Ok(IxMeta {
            name: name.to_owned(),
            gos_meta,
            process_method,
            process_method_index,
            accounts: accounts.into_iter().map(|(acc, _)| acc).collect(),
            accounts_data,
            args,
        })
    }

    fn get_data_access_mode(tag: &Option<String>, index: usize) -> Result<AccessMode> {
        if let Some(tag) = tag {
            let tag = tag.trim();
            if tag == "init" {
                Ok(AccessMode::Initialize(index))
            } else if tag == "mut" {
                Ok(AccessMode::Mutable(index))
            } else {
                Err(error!(GolError::BadDataDeclareTag))
            }
        } else {
            Ok(AccessMode::ReadOnly(index))
        }
    }

    fn is_signer_or_mut(tag: &Option<String>) -> (bool, bool) {
        if let Some(tag) = tag {
            let tags: Vec<&str> = tag.split(',').map(|x| x.trim()).collect();
            let is_signer = tags.contains(&"signer");
            let is_mut = tags.contains(&"mut");
            return (is_signer, is_mut);
        }
        (false, false)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct TxMeta {
    pub instructions: Vec<IxMeta>,
}

pub fn check(bc: &Bytecode) -> Result<TxMeta> {
    let account_info_meta = get_account_info_meta(bc).ok_or(error!(GolError::MetaNotFound))?;
    let main_pkg = &bc.objects.packages[bc.main_pkg];
    let ix_details: Vec<(&String, types::Meta)> = main_pkg
        .member_indices()
        .iter()
        .filter_map(|(name, index)| {
            (name.starts_with("Ix") && main_pkg.member(*index).typ() == types::ValueType::Metadata)
                .then(|| {
                    let member = main_pkg.member(*index);
                    let gmeta = member.as_metadata();
                    (name, gmeta.clone())
                })
        })
        .collect();

    let instructions = ix_details
        .into_iter()
        .map(|(name, meta)| IxMeta::new(name, meta, &account_info_meta, &bc.objects.metas))
        .collect::<Result<Vec<IxMeta>>>()?;
    Ok(TxMeta { instructions })
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
    let account = pkg.member(*pkg.member_index("AccountInfo")?);
    match account.typ() {
        types::ValueType::Metadata => Some(account.as_metadata().clone()),
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
