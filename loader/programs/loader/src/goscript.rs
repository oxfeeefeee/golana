use crate::ffi::{fmt2, solana};
use anchor_lang::prelude::*;
use borsh::de::BorshDeserialize;
use golana::*;
use goscript_vm::types::{GosValue, ValueType};
use goscript_vm::*;
use solana_program::account_info::AccountInfo;
use std::cell::RefCell;

pub fn run(
    bin: &Vec<u8>,
    meta_bin: &Vec<u8>,
    accounts: &[AccountInfo],
    id: &str,
    args: Vec<u8>,
) -> Result<()> {
    let bc = Bytecode::try_from_slice(bin).unwrap();
    let metas = TxMeta::try_from_slice(meta_bin).unwrap();
    let ix = Instruction::new(&metas, accounts, id, &args)?;
    let p = std::ptr::addr_of!(ix) as usize;

    let mut ffi = goscript_vm::FfiFactory::with_user_data(p);
    fmt2::Fmt2Ffi::register(&mut ffi);
    solana::SolanaFfi::register(&mut ffi);
    goscript_vm::run(&bc, &ffi, None);

    ix.write_back()?;
    Ok(())
}

pub(crate) struct Instruction<'a, 'info> {
    accounts: &'a [AccountInfo<'info>],
    args: &'a Vec<u8>,
    ix_meta: &'a IxMeta,
    gos_ix: RefCell<Option<GosValue>>,
}

impl<'a, 'info> Instruction<'a, 'info>
where
    'info: 'a,
{
    fn new(
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
            accounts,
            args,
            ix_meta,
            gos_ix: RefCell::new(None),
        })
    }

    pub(crate) fn get_ix(&self, ctx: &mut FfiCtx) -> GosValue {
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

    fn write_back(&self) -> Result<()> {
        let borrowed = self.gos_ix.borrow();
        let gos_ix = borrowed.as_ref().unwrap();
        let fields: &Vec<GosValue> = &gos_ix.as_struct().0.borrow_fields();
        let data_fields = &fields[self.accounts.len()..];
        for (i, _) in self.ix_meta.accounts_data.iter() {
            match self.ix_meta.accounts[*i].access_mode {
                AccessMode::Initialize | AccessMode::Mutable => {
                    let mut buf: &mut [u8] = &mut self.accounts[*i].data.borrow_mut();
                    GosValue::serialize_wo_type(&data_fields[*i], &mut buf)?;
                }
                _ => {}
            };
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
            if (acc_meta.access_mode == AccessMode::Mutable
                || acc_meta.access_mode == AccessMode::Initialize)
                != account.is_writable
            {
                return Err(error!(GolError::RtCheckMutable));
            }
            fields.push(Self::make_account_info(ctx, account));
        }
        for (i, data_meta) in self.ix_meta.accounts_data.iter() {
            let data = match self.ix_meta.accounts[*i].access_mode {
                AccessMode::Initialize => ctx.zero_val(data_meta),
                _ => {
                    let mut buf: &[u8] = &self.accounts[*i].data.borrow();
                    GosValue::deserialize_wo_type(data_meta, &ctx.vm_objs.metas, &mut buf)?
                }
            };
            fields.push(data);
        }
        let mut buf: &[u8] = &self.args;
        for arg_meta in self.ix_meta.args.iter() {
            fields.push(GosValue::deserialize_wo_type(
                arg_meta,
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
        FfiCtx::new_interface(ix, Some((ix_meta.gos_meta, binding)))
    }

    fn make_account_info(ctx: &FfiCtx, ai: &AccountInfo) -> GosValue {
        let key = Self::make_pub_key(ctx, ai.key);
        let lamports: GosValue = (**ai.lamports.borrow()).into();
        let owner = Self::make_pub_key(ctx, ai.owner);
        let executable: GosValue = ai.executable.into();
        let rent_epoch: GosValue = ai.rent_epoch.into();
        ctx.new_struct(vec![key, lamports, owner, executable, rent_epoch])
    }

    #[inline]
    fn make_pub_key(ctx: &FfiCtx, key: &Pubkey) -> GosValue {
        ctx.new_1byte_array(key.to_bytes().to_vec(), ValueType::Uint8)
    }
}
