use crate::error::CompilerError;

use super::{Context, Declaration, Expression};

#[derive(Debug)]
pub enum Statement {
    Compound(Box<[Statement]>),
    Declaration(Declaration),
    Expression(Box<Expression>),
    If(Box<IfStatement>),
    WhileLoop(Box<WhileLoopStatement>),
    Return(Box<Option<Expression>>),

    // TODO: 
    // - Labels & Jumps
    // - Do-Whiles and Fors
    // - Break, Continue
    // - Switches
}

// If-Else can be done by chaining these
#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub if_block: Statement,
    pub else_block: Option<Statement>,
}

#[derive(Debug)]
pub struct WhileLoopStatement {
    pub condition: Expression,
    pub body: Statement,
}

impl Statement {
    pub fn verify(&self, context: &mut Context) -> Result<(), CompilerError> {

        // TODO
        Ok(())
    }
}
