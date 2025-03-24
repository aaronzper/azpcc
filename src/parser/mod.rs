use lalrpop_util::lalrpop_mod;

use crate::{ast::{statements::{IfStatement, WhileLoopStatement}, Expression, Statement, TranslationUnit}, error::CompilerError};

lalrpop_mod!(grammar, "/parser/grammar.rs");

// Helpers for LALRPOP grammar
pub fn parse_if(
    condition: Expression,
    if_block: Statement,
    else_block: Option<Statement>
) -> Statement {
    Statement::If(Box::new(IfStatement { condition, if_block, else_block }))
}

pub fn parse_while(condition: Expression, body: Statement) -> Statement {
    Statement::WhileLoop(Box::new(WhileLoopStatement { condition, body } ))
}

pub fn parse(input: &str) -> Result<TranslationUnit, CompilerError> {
    let output = grammar::TransalationUnitParser::new().parse(input)?;
    Ok(output)
}
