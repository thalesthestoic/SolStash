use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the Token vesting program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum StashError {
    // Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction
}

impl From<StashError> for ProgramError {
    fn from(e: StashError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for StashError {
    fn type_of() -> &'static str {
        "VestingError"
    }
}