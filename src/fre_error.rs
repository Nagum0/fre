use std::ffi::OsString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FreError {
    #[error("fre: {:?}: INVALID DATA", .0)]
    InvalidData(OsString),
}
