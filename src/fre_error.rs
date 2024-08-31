use std::ffi::OsString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FreError<'a> {
    #[error("fre: {:?}: INVALID DATA", ._0)]
    InvalidData(&'a OsString),
    #[error("fre: {:?}: FILE ERROR: {}", ._0, ._1)]
    FileError(&'a OsString, &'a str),
    #[error("fre: {:?}: DIRECTORY ERROR: {}", ._0, ._1)]
    DirError(&'a OsString, &'a str),
    #[error("fre: ARGUMENT ERROR: Expected {} arguments but received {}", ._0, ._1)]
    ArgError(usize, usize),
    #[error("fre: UNKNOWN OPTION ERROR: {}", ._0)]
    UnknownFlagError(String),
}
