use crate::ffi::{fmt2, math2, solana, token};
use anchor_lang::prelude::*;
use go_vm::types::GosValue;
use go_vm::*;
use golana::*;
use solana_program::account_info::AccountInfo;
use std::cell::RefCell;

pub fn run(
    key: &Pubkey,
    bc: &Bytecode,
    metas: &TxMeta,
    accounts: &[AccountInfo],
    id: &str,
    args: Vec<u8>,
) -> Result<()> {
    let ix = Instruction::new(key, &metas, accounts, id, &args)?;
    let p = std::ptr::addr_of!(ix) as usize;

    let mut ffi = go_vm::FfiFactory::with_user_data(p);
    fmt2::Fmt2Ffi::register(&mut ffi);
    math2::Math2Ffi::register(&mut ffi);
    solana::SolanaFfi::register(&mut ffi);
    token::TokenFfi::register(&mut ffi);
    go_vm::run(&bc, &ffi, None);

    Ok(())
}

pub(crate) struct Instruction<'a, 'info> {
    pub gos_program_id: &'a Pubkey,
    pub accounts: &'a [AccountInfo<'info>],
    pub args: &'a Vec<u8>,
    pub ix_meta: &'a IxMeta,
    pub gos_ix: RefCell<Option<GosValue>>,
}

impl<'a, 'info> Instruction<'a, 'info>
where
    'info: 'a,
{
    fn new(
        gos_program_id: &'a Pubkey,
        tx_meta: &'a TxMeta,
        accounts: &'a [AccountInfo<'info>],
        id: &'a str,
        args: &'a Vec<u8>,
    ) -> Result<Instruction<'a, 'info>> {
        let ix_meta = tx_meta
            .instructions
            .iter()
            .find(|x| x.name == id)
            .ok_or(error!(GolError::RtCheckBadIxId))?;
        if ix_meta.accounts.len() != accounts.len() {
            return Err(error!(GolError::RtCheckAccountCount));
        }

        Ok(Instruction {
            gos_program_id,
            accounts,
            args,
            ix_meta,
            gos_ix: RefCell::new(None),
        })
    }

    pub(crate) fn get_ix(&self, ctx: &FfiCtx) -> GosValue {
        let gos_ix: &mut Option<GosValue> = &mut self.gos_ix.borrow_mut();
        match gos_ix {
            Some(val) => val.clone(),
            None => {
                let ix = self.deserialize_ix(ctx).unwrap();
                *gos_ix = Some(ix.clone());
                ix
            }
        }
    }

    pub(crate) fn write_back_data(
        &self,
        ctx: &FfiCtx,
        indices: std::ops::Range<usize>,
        lamports: bool,
        data: bool,
    ) -> Result<()> {
        let borrowed = self.gos_ix.borrow();
        let gos_ix = borrowed
            .as_ref()
            .unwrap()
            .as_non_nil_interface()
            .unwrap()
            .underlying_value()
            .unwrap();
        let gos_ix = ctx.deref_pointer(&gos_ix).unwrap();
        let ix_fields: &Vec<GosValue> = &gos_ix.as_struct().0.borrow_fields();
        for index in indices {
            let account_info = ctx.deref_pointer(&ix_fields[index]).unwrap();
            let account_info_fields: &Vec<GosValue> = &account_info.as_struct().0.borrow_fields();
            if lamports {
                let val = *account_info_fields[1].as_uint64();
                **self.accounts[index].lamports.borrow_mut() = val;
            }
            if data {
                let data_fields = &ix_fields[self.accounts.len()..];
                let account_meta = &self.ix_meta.accounts[index];
                if let Some(data_index) = account_meta.access_mode.get_data_index() {
                    let mut buf: &mut [u8] = &mut self.accounts[index].data.borrow_mut();
                    let data_obj = ctx.deref_pointer(&data_fields[data_index]).unwrap();
                    GosValue::serialize_wo_type(&data_obj, &mut buf)?;
                }
            }
        }
        Ok(())
    }

    fn deserialize_ix(&self, ctx: &FfiCtx) -> Result<GosValue> {
        let mut fields = vec![];
        for (i, acc_meta) in self.ix_meta.accounts.iter().enumerate() {
            let account = &self.accounts[i];
            if acc_meta.is_signer != account.is_signer {
                return Err(error!(GolError::RtCheckSigner));
            }
            // if acc_meta.access_mode.is_writable() != account.is_writable {
            //     return Err(error!(GolError::RtCheckMutable));
            // }
            fields.push(solana::SolanaFfi::make_account_info_ptr(ctx, account, i));
        }
        for (i, data_meta) in self.ix_meta.accounts_data.iter() {
            let data = match self.ix_meta.accounts[*i].access_mode {
                AccessMode::Initialize(_) => ctx.zero_val(data_meta),
                _ => {
                    let mut buf: &[u8] = &self.accounts[*i].data.borrow();
                    GosValue::deserialize_wo_type(data_meta, &ctx.vm_objs.metas, &mut buf)?
                }
            };
            fields.push(FfiCtx::new_pointer(data));
        }
        let mut buf: &[u8] = &self.args;
        for arg_meta in self.ix_meta.args.iter() {
            fields.push(GosValue::deserialize_wo_type(
                &arg_meta.1,
                &ctx.vm_objs.metas,
                &mut buf,
            )?);
        }

        Ok(Self::make_interface(ctx.new_struct(fields), self.ix_meta))
    }

    fn make_interface(ix: GosValue, ix_meta: &IxMeta) -> GosValue {
        let binding = vec![types::Binding4Runtime::Struct(
            ix_meta.process_method,
            true,
            None,
        )];
        FfiCtx::new_interface(
            FfiCtx::new_pointer(ix),
            Some((ix_meta.gos_meta.ptr_to(), binding)),
        )
    }
}
