use lalrpop_util::lalrpop_mod;

use crate::{ast::TranslationUnit, error::CompilerError};

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub fn parse(input: &str) -> Result<TranslationUnit, CompilerError> {
    let output = grammar::TransalationUnitParser::new().parse(input)?;
    Ok(output)
}
