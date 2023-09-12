use anchor_syn::idl;
use anyhow::{anyhow, Result};
use go_vm::types::{Meta, MetadataObjs, MetadataType, ValueType};
use golana;

pub struct IdlGen<'a> {
    meta_objs: &'a MetadataObjs,
    tx_meta: &'a golana::TxMeta,
}

impl IdlGen<'_> {
    pub fn new<'a>(meta_objs: &'a MetadataObjs, tx_meta: &'a golana::TxMeta) -> IdlGen<'a> {
        IdlGen { meta_objs, tx_meta }
    }

    pub fn gen(&self, proj_name: &str) -> Result<idl::Idl> {
        let instructions = self
            .tx_meta
            .instructions
            .iter()
            .map(|inst| self.get_ix_idl(inst))
            .collect::<Result<Vec<idl::IdlInstruction>>>()?;
        Ok(idl::Idl {
            version: "0.0.0".to_owned(),
            name: proj_name.to_owned(),
            docs: None,
            instructions,
            accounts: vec![],
            constants: vec![],
            types: vec![],
            errors: None,
            events: None,
            metadata: None,
        })
    }

    fn get_ix_idl(&self, ix: &golana::IxMeta) -> Result<idl::IdlInstruction> {
        Ok(idl::IdlInstruction {
            name: ix.name.clone(),
            docs: None,
            accounts: ix
                .accounts
                .iter()
                .map(|acc| idl::IdlAccountItem::IdlAccount(Self::get_account_idl(acc)))
                .collect(),
            args: ix
                .args
                .iter()
                .map(|arg| self.get_arg_idl(arg))
                .collect::<Result<Vec<idl::IdlField>>>()?,
            returns: None,
        })
    }

    fn get_arg_idl(&self, arg: &(String, Meta)) -> Result<idl::IdlField> {
        Ok(idl::IdlField {
            name: arg.0.clone(),
            docs: None,
            ty: self.get_idl_type(&arg.1)?,
        })
    }

    fn get_account_idl(acc: &golana::AccMeta) -> idl::IdlAccount {
        idl::IdlAccount {
            name: acc.name.clone(),
            is_signer: acc.is_signer,
            is_mut: acc.is_mut,
            is_optional: None,
            docs: None,
            pda: None,
            relations: vec![],
        }
    }

    fn get_idl_type(&self, typ: &Meta) -> Result<idl::IdlType> {
        match self.meta_objs[typ.key] {
            MetadataType::Array(t, size) => Ok(idl::IdlType::Array(
                Box::new(self.gos_type_to_idl_type(&t)?),
                size,
            )),
            MetadataType::Slice(t) => {
                if t.value_type(self.meta_objs) == ValueType::Uint8 {
                    Ok(idl::IdlType::Bytes)
                } else {
                    Ok(idl::IdlType::Vec(Box::new(self.gos_type_to_idl_type(&t)?)))
                }
            }
            MetadataType::Named(_, inner) => {
                if typ == &self.tx_meta.pub_key_meta {
                    Ok(idl::IdlType::PublicKey)
                } else {
                    self.get_idl_type(&inner)
                }
            }
            _ => self.gos_type_to_idl_type(typ),
        }
    }

    fn gos_type_to_idl_type(&self, typ: &Meta) -> Result<idl::IdlType> {
        let vt = typ.value_type(self.meta_objs);
        match vt {
            ValueType::Uint8 => Ok(idl::IdlType::U8),
            ValueType::Uint16 => Ok(idl::IdlType::U16),
            ValueType::Uint32 => Ok(idl::IdlType::U32),
            ValueType::Uint64 => Ok(idl::IdlType::U64),
            ValueType::Int8 => Ok(idl::IdlType::I8),
            ValueType::Int16 => Ok(idl::IdlType::I16),
            ValueType::Int32 => Ok(idl::IdlType::I32),
            ValueType::Int64 => Ok(idl::IdlType::I64),
            ValueType::Float32 => Ok(idl::IdlType::F32),
            ValueType::Float64 => Ok(idl::IdlType::F64),
            ValueType::Bool => Ok(idl::IdlType::Bool),
            ValueType::String => Ok(idl::IdlType::String),
            _ => Err(anyhow!("Unsupported type: {}", vt)),
        }
    }
}
