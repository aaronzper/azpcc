use std::io;
use lalrpop_util::{lexer::Token, ParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("File Error: {0}")]
    FileError(#[from] io::Error),

    #[error("Parsing Error: {0}")]
    ParseError(
        lalrpop_util::ParseError<usize, String, &'static str>),

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

/// All this is doing is taking ownership of the tokens passed back by LALRPOP's
/// errors. They're all `&'a str`s, but we want `String`s!
impl<'a> From<ParseError<usize, Token<'a>, &'static str>> for CompilerError {
    fn from(value: ParseError<usize, Token<'a>, &'static str>) -> Self {
        let new = match value {
            ParseError::InvalidToken { location } =>
                ParseError::InvalidToken { location },
            ParseError::UnrecognizedEof { location, expected } =>
                ParseError::UnrecognizedEof { location, expected },
            ParseError::UnrecognizedToken { token, expected } => {
                let (start, tok, end) = token;
                let new_tok = String::from(tok.1);
                ParseError::UnrecognizedToken {
                    token: (start, new_tok, end),
                    expected
                }
            },
            ParseError::ExtraToken { token } => {
                let (start, tok, end) = token;
                let new_tok = String::from(tok.1);
                ParseError::ExtraToken {
                    token: (start, new_tok, end)
                }
            },
            ParseError::User { error } => ParseError::User { error }
        };

        CompilerError::ParseError(new)
    }
}
