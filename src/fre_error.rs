use std::ffi::OsString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FreError<'a> {
    #[error("fre: {:?}: INVALID DATA", ._0)]
    InvalidData(&'a OsString),
    #[error("fre: {:?}: {}", ._0, ._1)]
    FileError(&'a OsString, &'a str),
}
