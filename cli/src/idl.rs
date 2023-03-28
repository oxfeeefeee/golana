use anchor_syn::idl;
use anyhow::{anyhow, Result};
use golana;
use goscript_vm::types::{Meta, MetadataObjs, MetadataType, ValueType};

pub fn get_idl(tx: &golana::TxMeta, metas: &MetadataObjs) -> Result<idl::Idl> {
    let instructions = tx
        .instructions
        .iter()
        .map(|inst| get_ix_idl(inst, metas))
        .collect::<Result<Vec<idl::IdlInstruction>>>()?;
    Ok(idl::Idl {
        version: "0.0.0".to_owned(),
        name: "unnamed".to_owned(),
        docs: None,
        instructions,
        accounts: vec![],
        constants: vec![],
        types: vec![],
        errors: None,
        events: None,
        state: None,
        metadata: None,
    })
}

fn get_ix_idl(ix: &golana::IxMeta, metas: &MetadataObjs) -> Result<idl::IdlInstruction> {
    Ok(idl::IdlInstruction {
        name: ix.name.clone(),
        docs: None,
        accounts: ix
            .accounts
            .iter()
            .map(|acc| idl::IdlAccountItem::IdlAccount(get_account_idl(acc)))
            .collect(),
        args: ix
            .args
            .iter()
            .map(|arg| get_arg_idl(arg, metas))
            .collect::<Result<Vec<idl::IdlField>>>()?,
        returns: None,
    })
}

fn get_arg_idl(arg: &(String, Meta), metas: &MetadataObjs) -> Result<idl::IdlField> {
    Ok(idl::IdlField {
        name: arg.0.clone(),
        docs: None,
        ty: get_idl_type(&arg.1, metas)?,
    })
}

fn get_account_idl(acc: &golana::AccMeta) -> idl::IdlAccount {
    idl::IdlAccount {
        name: acc.name.clone(),
        is_signer: acc.is_signer,
        is_mut: acc.is_mut,
        docs: None,
        pda: None,
    }
}

fn get_idl_type(typ: &Meta, metas: &MetadataObjs) -> Result<idl::IdlType> {
    match metas[typ.key] {
        MetadataType::Array(t, size) => Ok(idl::IdlType::Array(
            Box::new(gos_type_to_idl_type(t.value_type(metas))?),
            size,
        )),
        MetadataType::Slice(t) => Ok(idl::IdlType::Vec(Box::new(gos_type_to_idl_type(
            t.value_type(metas),
        )?))),
        _ => gos_type_to_idl_type(typ.value_type(metas)),
    }
}

fn gos_type_to_idl_type(t: ValueType) -> Result<idl::IdlType> {
    match t {
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
        _ => Err(anyhow!("Unsupported type: {}", t)),
    }
}
