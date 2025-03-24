use crate::error::CompilerError;

use super::{Expression, Statement, Type};

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub type_of: Type,
    pub value: Option<DeclarationValue>,
}

#[derive(Debug)]
pub enum DeclarationValue {
    Variable(Expression),
    Function(Box<[Statement]>),
}

impl Declaration {
    pub fn verify(&self, context: &mut super::Context) -> 
            Result<(), CompilerError> {
    
        context.add_name(self.name.clone(), self.type_of.clone())?;

        match &self.value {
            // If its just a declaration (not definition), nothing to verify
            None => (),

            Some(DeclarationValue::Variable(expr)) => {
                let is_global = context.return_type().is_none();

                if is_global {
                    match expr {
                        Expression::IntLiteral(_) => (),
                        Expression::CharLiteral(_) => (),
                        Expression::StringLiteral(_) => (),
                        _ => return Err(CompilerError::SemanticError("Global variable assignment must be a literal\nConstant folding isn't currently supported")),
                    }
                } else {
                    expr.verify(context)?;
                }
            },

            Some(DeclarationValue::Function(stmts)) => {
                let mut inner = context.inner();

                if let Type::Function(f) = &self.type_of {
                    inner.set_return_type(f.return_type.clone())?;

                    for (arg_n, arg_t) in &f.args {
                        inner.add_name(arg_n.clone(), arg_t.clone())?;
                    }

                    for stmt in stmts {
                        stmt.verify(&mut inner)?;
                    }
                } else {
                    panic!("Encountered weird enum varient");
                }
            }
        };

        Ok(())
    }
}

