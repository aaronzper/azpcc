use crate::error::CompilerError;

use super::{Context, Declaration, Expression, Type};

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

impl IfStatement {
    fn verify(&self, context: &mut Context) -> Result<(), CompilerError> {
        if !self.condition.verify(context)?.is_integer() {
            return Err(CompilerError::SemanticError("If condition must resolve to an integer type"));
        }

        self.if_block.verify(context)?;

        if let Some(else_stmt) = &self.else_block {
            else_stmt.verify(context)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct WhileLoopStatement {
    pub condition: Expression,
    pub body: Statement,
}

impl WhileLoopStatement {
    fn verify(&self, context: &mut Context) -> Result<(), CompilerError> {
        if !self.condition.verify(context)?.is_integer() {
            return Err(CompilerError::SemanticError("While condition must resolve to an integer type"));
        }

        self.body.verify(context)?;

        Ok(())
    }
}

fn verify_compound(stmts: &Box<[Statement]>, context: &mut Context) ->
    Result<(), CompilerError> {

    let mut inner = context.inner();

    for stmt in stmts {
        stmt.verify(&mut inner)?;
    }

    Ok(())
}

fn verify_return(expr: &Option<Expression>, context: &mut Context) ->
    Result<(), CompilerError> {

    let expected_type = context
        .return_type()
        .ok_or(CompilerError::SemanticError("Cannot return from outside of a function"))?
        .clone();

    match expr {
        Some(ex) => {
            if expected_type == Type::Void {
                return Err(CompilerError::SemanticError("Cannot return value from void fn"));
            } 

            let actual_type = ex.verify(context)?;
            if expected_type != actual_type {
                return Err(CompilerError::SemanticError("Tried to return incorrect type"));
            }
        },

        None => {
            if expected_type != Type::Void {
                return Err(CompilerError::SemanticError("Cannot return void from non-void fn"));
            }
        },
    }

    Ok(())
}

impl Statement {
    pub fn verify(&self, context: &mut Context) -> Result<(), CompilerError> {
        match self {
            Self::Compound(stmts) => verify_compound(stmts, context)?,
            Self::Declaration(decl) => decl.verify(context)?,
            Self::Expression(expr) => { expr.verify(context)?; },
            Self::If(x) => x.verify(context)?,
            Self::WhileLoop(x) => x.verify(context)?,
            Self::Return(expr) => verify_return(expr.as_ref(), context)?,
        };

        Ok(())
    }
}
