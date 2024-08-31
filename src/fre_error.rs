use std::ffi::OsString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FreError {
    #[error("fre: {:?}: INVALID DATA", ._0)]
    InvalidData(OsString),
    #[error("fre: {:?}: FILE ERROR: {}", ._0, ._1)]
    FileError(OsString, &'static str),
    #[error("fre: {:?}: DIRECTORY ERROR: {}", ._0, ._1)]
    DirError(OsString, &'static str),
    #[error("fre: ARGUMENT ERROR: Expected {} arguments but received {}", ._0, ._1)]
    ArgError(usize, usize),
    #[error("fre: UNKNOWN OPTION ERROR: {}", ._0)]
    UnknownFlagError(String),
}
