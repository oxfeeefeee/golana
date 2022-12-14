use anchor_lang::prelude::*;

#[error_code]
pub enum GolError {
    #[msg("Handle doesn't match against the public key")]
    WrongHandle,
    #[msg("No instruction found, which should be a struct in main package named Ix... ")]
    IxNotFound,
    #[msg("AccountInfo meta is not found, 'solana' package not imported?")]
    MetaNotFound,
    #[msg("Method 'process' for the Ix struct is not found")]
    MethodNotFound,
    #[msg("Method 'process' has to have a pointer receiver")]
    NonPointerReceiver,
    #[msg("AccountInfo in Ix has to be a pointer")]
    NonPointerAccountInfo,
    #[msg("AccountInfo in Ix has to be a pointer")]
    NonPointerDataDeclare,
    #[msg("Duplicated Account data declaration")]
    DuplicatedDataDeclare,
    #[msg("Bad Account data declaration")]
    BadDataDeclare,
    #[msg("This argument type is not supported")]
    WrongArgType,
    #[msg("No instruction found with provided ID")]
    RtCheckBadIxId,
    #[msg("Unexpected account count provided")]
    RtCheckAccountCount,
    #[msg("A signer account is expected")]
    RtCheckSigner,
    #[msg("A mutable account is expected")]
    RtCheckMutable,

    #[msg("Bad AuthorityType value")]
    BadAuthorityType,
}
