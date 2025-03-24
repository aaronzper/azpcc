// Helpers for LALRPOP grammar

use crate::ast::{expressions::{BinaryExpr, UnaryExpr}, statements::{IfStatement, WhileLoopStatement}, Expression, Statement};

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

pub fn parse_unary(expr: Expression) -> Box<UnaryExpr> {
    Box::new(UnaryExpr { expr })
}

pub fn parse_binary(first: Expression, second: Expression) -> Box<BinaryExpr> {
    Box::new(BinaryExpr { first, second })
}
