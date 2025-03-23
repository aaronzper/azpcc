use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("File Error: {0}")]
    FileError(#[from] io::Error),

    #[error("Feature Not Supported: {0}")]
    NotSupported(&'static str),

    #[error("{0}")]
    Custom(&'static str),
}

impl From<&'static str> for CompilerError {
    fn from(value: &'static str) -> Self {
        Self::Custom(value)
    }
}
