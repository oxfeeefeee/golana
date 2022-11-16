use anchor_lang::prelude::*;

#[error_code]
pub enum GolError {
    #[msg("Handle doesn't match against the public key")]
    WrongHandle,
    #[msg("No instruction found, which should be a struct in main package named Ix... ")]
    IxNotFound,
    #[msg("AccountInfo meta is not found, 'solana' package not imported?")]
    MetaNotFound,
    #[msg("Cannot find corresponding AccountInfo for the Data declaration")]
    WrongDataDeclare,
    #[msg("This argument type is not supported")]
    WrongArgType,
}
