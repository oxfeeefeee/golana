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
    let ix = Instruction::new(key, &metas.iface_meta, &metas, accounts, id, &args)?;
    let p = std::ptr::addr_of!(ix) as usize;

    let mut ffi = go_vm::FfiFactory::with_user_data(p);
    fmt2::Fmt2Ffi::register(&mut ffi);
    math2::Math2Ffi::register(&mut ffi);
    solana::SolanaFfi::register(&mut ffi);
    token::TokenFfi::register(&mut ffi);

    let panic_info = go_vm::run(&bc, &ffi);
    if let Some(pi) = panic_info {
        let call_stack = go_vm::CallStackDisplay::new(&pi, bc);
        msg!("GolanaVM panic: {}\n", pi.msg);
        msg!("Call stack:\n{}", call_stack);
        panic!("panic from go");
    }
    Ok(())
}

pub(crate) struct Instruction<'a, 'info> {
    pub gos_program_id: &'a Pubkey,
    pub accounts: &'a [AccountInfo<'info>],
    pub args: &'a Vec<u8>,
    pub iface_meta: &'a types::Meta,
    pub ix_meta: &'a IxMeta,
    pub gos_ix: RefCell<Option<GosValue>>,
}

impl<'a, 'info> Instruction<'a, 'info>
where
    'info: 'a,
{
    fn new(
        gos_program_id: &'a Pubkey,
        iface_meta: &'a types::Meta,
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
            iface_meta,
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

    fn deserialize_ix(&self, ctx: &FfiCtx) -> Result<GosValue> {
        let mut fields = vec![];
        for (i, acc_meta) in self.ix_meta.accounts.iter().enumerate() {
            let account = &self.accounts[i];
            if acc_meta.is_signer != account.is_signer {
                return Err(error!(GolError::RtCheckSigner));
            }
            if acc_meta.is_mut != account.is_writable {
                return Err(error!(GolError::RtCheckMutable));
            }
            fields.push(i.into());
        }

        let mut buf: &[u8] = &self.args;
        for arg_meta in self.ix_meta.args.iter() {
            fields.push(GosValue::deserialize_wo_type(
                &arg_meta.1,
                &ctx.vm_objs.metas,
                &mut buf,
            )?);
        }

        let ix = ctx.new_struct(fields);
        Ok(ctx.new_interface(
            FfiCtx::new_pointer(ix),
            Some((self.iface_meta, self.ix_meta.gos_meta.ptr_to())),
        ))
    }
}
