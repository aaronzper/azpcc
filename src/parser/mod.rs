use lalrpop_util::lalrpop_mod;

use crate::{ast::{statements::{IfStatement, WhileLoopStatement}, Expression, Statement, TranslationUnit}, error::CompilerError};

pub mod helpers;

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub fn parse(input: &str) -> Result<TranslationUnit, CompilerError> {
    let output = grammar::TransalationUnitParser::new().parse(input)?;
    Ok(output)
}
