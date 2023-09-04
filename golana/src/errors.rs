use anchor_lang::prelude::*;

#[error_code]
pub enum GolError {
    #[msg("Handle doesn't match against the public key")]
    WrongHandle,
    #[msg("Handle is too long")]
    HandleTooLong,
    #[msg("No instruction found, which should be a struct in main package named Ix... ")]
    IxNotFound,
    #[msg("AccountInfo meta is not found, 'solana' package not imported?")]
    MetaNotFound,
    #[msg("Method 'process' for the Ix struct is not found")]
    MethodNotFound,
    #[msg("Method 'process' has to have a pointer receiver")]
    NonPointerReceiver,
    #[msg("Account in Ix cannot be a pointer")]
    PointerAccount,
    #[msg("Account Data in Ix has to be a pointer")]
    NonPointerDataDeclare,
    #[msg("Account name can only be used as a prefix for data declaration")]
    AccountNamePrefixReserved,
    #[msg("Data type specified in the tag is not found")]
    DataTypeNotFound,
    #[msg("Data type not specified in the tag of the account")]
    DataTypeNotSpecified,
    #[msg("This argument type is not supported")]
    WrongArgType,
    #[msg("Unexpected finalize step")]
    WrongFinalizeStep,
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
