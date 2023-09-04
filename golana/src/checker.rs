use crate::errors::*;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use go_vm::{types::PackageObj, *};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub enum AccessMode {
    None,
    Initialize(usize),
    ReadOnly(usize),
    Mutable(usize),
}

impl AccessMode {
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
    pub data_meta: Option<types::Meta>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct IxMeta {
    pub name: String,
    pub gos_meta: types::Meta,
    pub process_method: types::FunctionKey,
    pub process_method_index: usize,
    pub accounts: Vec<AccMeta>,
    pub args: Vec<(String, types::Meta)>,
}

impl IxMeta {
    fn new(
        name: &str,
        gos_meta: types::Meta,
        account: &types::Meta,
        pkg: &types::PackageObj,
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
            let account_tag = &fields[i].lookup_tag("account");
            let (is_signer, is_mut) = Self::is_signer_or_mut(account_tag);
            let data_tag = &fields[i].lookup_tag("data");
            let data_meta = Self::get_data_type(data_tag, pkg)?;
            if meta.key == account.key {
                if meta.ptr_depth != 0 {
                    return Err(error!(GolError::PointerAccount));
                }
                accounts.push((
                    AccMeta {
                        name: name.to_owned(),
                        is_signer,
                        is_mut,
                        data_meta,
                    },
                    &fields[i].name,
                ));
                i += 1;
            } else {
                break;
            }
        }

        // Then arguments
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
            args,
        })
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

    fn get_data_type(tag: &Option<String>, pkg: &PackageObj) -> Result<Option<types::Meta>> {
        match tag {
            Some(t) => {
                let index = pkg
                    .member_index(&t)
                    .ok_or(error!(GolError::DataTypeNotFound))?;
                let meta = pkg.member(*index);
                if meta.typ() != types::ValueType::Metadata {
                    return Err(error!(GolError::DataTypeNotFound));
                }
                Ok(Some(meta.as_metadata().clone()))
            }
            None => Ok(None),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct TxMeta {
    pub iface_meta: types::Meta,
    pub instructions: Vec<IxMeta>,
}

pub fn check(bc: &Bytecode) -> Result<TxMeta> {
    let account_meta = get_account_meta(bc).ok_or(error!(GolError::MetaNotFound))?;

    let mut iface_meta = None;
    let mut ix_details = Vec::new();
    for pkg in bc.objects.packages.iter() {
        if pkg.name() == "solana" {
            // Find the interface metadata in solana package
            for (name, index) in pkg.member_indices() {
                if name == "Ix" && pkg.member(*index).typ() == types::ValueType::Metadata {
                    iface_meta = Some(pkg.member(*index).as_metadata().clone());
                }
            }
        } else {
            for (name, index) in pkg.member_indices() {
                if name.starts_with("Ix") && pkg.member(*index).typ() == types::ValueType::Metadata
                {
                    let member = pkg.member(*index);
                    let gmeta = member.as_metadata();
                    ix_details.push((name, gmeta.clone(), pkg));
                }
            }
        }
    }

    let instructions = ix_details
        .into_iter()
        .map(|(name, meta, pkg)| IxMeta::new(name, meta, &account_meta, pkg, &bc.objects.metas))
        .collect::<Result<Vec<IxMeta>>>()?;
    Ok(TxMeta {
        iface_meta: iface_meta.unwrap(),
        instructions,
    })
}

fn get_account_meta(bc: &Bytecode) -> Option<types::Meta> {
    let key = bc
        .objects
        .packages
        .iter()
        .enumerate()
        .find(|(_, pkg)| pkg.name() == "solana")
        .map(|(i, _)| i.into())?;
    let pkg = &bc.objects.packages[key];
    let account = pkg.member(*pkg.member_index("Account")?);
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
