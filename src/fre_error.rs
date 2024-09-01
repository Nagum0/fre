use std::ffi::OsString;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FreError {
    #[error("fre: {:?}: INVALID DATA", ._0)]
    InvalidData(Rc<OsString>),
    #[error("fre: {:?}: FILE ERROR: {}", ._0, ._1)]
    FileError(Rc<OsString>, &'static str),
    #[error("fre: {:?}: DIRECTORY ERROR: {}", ._0, ._1)]
    DirError(Rc<OsString>, &'static str),
    #[error("fre: ARGUMENT ERROR: Expected {} arguments but received {}", ._0, ._1)]
    ArgError(usize, usize),
    #[error("fre: UNKNOWN OPTION ERROR: {}", ._0)]
    UnknownFlagError(String),
}
