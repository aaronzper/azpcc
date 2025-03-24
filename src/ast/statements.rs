use super::{Declaration, Expression, SemanticUnit};

#[derive(Debug)]
pub enum Statement {
    Compound(Box<[Statement]>),
    Expression(Box<Expression>),
    If(Box<IfStatement>),
    WhileLoop(Box<WhileLoopStatement>),
    Return(Box<Expression>),

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

impl SemanticUnit for Statement {
    fn verify_with_context(&self, context: &mut super::Context) -> 
        Result<(), crate::error::CompilerError> {

        // TODO
        Ok(())
    }
}
