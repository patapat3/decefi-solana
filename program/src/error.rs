use solana_sdk::{
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
use num_enum::{FromPrimitive, IntoPrimitive};

#[repr(u8)]
#[derive(Error, Debug)]
pub enum SourceFileId {
    #[error("src/account.rs")]
    Account = 1,
    #[error("src/decefi.rs")]
    Decefi = 2,
}

#[derive(Debug)]
pub struct AssertionError {
    pub line: u16,
    pub file_id: SourceFileId,
}

impl From<AssertionError> for u32 {
    fn from(err: AssertionError) -> u32 {
        (err.line as u32) + ((err.file_id as u8 as u32) << 24)
    }
}

impl From<AssertionError> for DecefiError {
    fn from(err: AssertionError) -> DecefiError {
        let err: u32 = err.into();
        DecefiError::ProgramError(ProgramError::Custom(err.into()))
    }
}

pub type DecefiResult<T = ()> = Result<T, DecefiError>;

#[derive(Error, Debug)]
pub enum DecefiError {
    #[error(transparent)]
    ProgramError(#[from] ProgramError),
    #[error("{0:?}")]
    ErrorCode(#[from] DecefiErrorCode),
}

#[derive(Debug, IntoPrimitive, FromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum DecefiErrorCode {
    InvalidHash = 0,
    InvalidAmount,
    CanceledByOracle,
    DeserializationFailed,
    NoPermisson,
    NoTrade,
    WrongInput,
    BorrowError,
    #[num_enum(default)]
    AssertionError,
}

impl std::fmt::Display for DecefiErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        <Self as std::fmt::Debug>::fmt(self, fmt)
    }
}

impl std::error::Error for DecefiErrorCode {}

impl std::convert::From<DecefiError> for ProgramError {
    fn from(e: DecefiError) -> ProgramError {
        match e {
            DecefiError::ProgramError(e) => e,
            DecefiError::ErrorCode(c) => ProgramError::Custom(c.into()),
        }
    }
}

impl std::convert::From<std::cell::BorrowError> for DecefiError {
    fn from(_: std::cell::BorrowError) -> Self {
        DecefiError::ErrorCode(DecefiErrorCode::BorrowError)
    }
}
