use anchor_lang::prelude::*;

#[error_code]
pub enum GolError {
    #[msg("No instruction found, which should be a struct in main package named Ix... ")]
    NoIxFound,
}
