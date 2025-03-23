use super::{Declaration, Expression};

pub enum Statement {
    Compound(Box<[Statement]>),
    Declaration(Box<Declaration>),
    Expression(Box<Expression>),
    If(Box<IfStatement>),
    WhileLoop(Box<WhileLoopStatement>),

    // TODO: 
    // - Labels & Jumps
    // - Do-Whiles and Fors
    // - Switches
}

// If-Else can be done by chaining these
pub struct IfStatement {
    pub condition: Expression,
    pub if_block: Statement,
    pub else_block: Option<Statement>,
}

pub struct WhileLoopStatement {
    pub condition: Expression,
    pub body: Statement,
}
