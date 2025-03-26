use log::trace;

use crate::error::CompilerError;

use super::{Context, Expression, Statement, Type};

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub type_of: Type,
    pub external: bool,
    pub value: Option<DeclarationValue>,
}

#[derive(Debug)]
pub enum DeclarationValue {
    Variable(Expression),
    Function(Box<[Statement]>),
}

impl Declaration {
    pub fn verify(&self, context: &mut Context) -> Result<(), CompilerError> {
        trace!("Semantically checking {:?}", self);

        context.add_name(self.name.clone(), self.type_of.clone())?;

        match (self.external, &self.value) {
            // If its just a declaration (not definition), nothing to verify
            (_, None) => (),

            (false, Some(DeclarationValue::Variable(expr))) => {
                let is_global = context.return_type().is_none();

                if is_global {
                    match expr {
                        Expression::IntLiteral(_) => (),
                        Expression::CharLiteral(_) => (),
                        Expression::StringLiteral(_) => (),
                        _ => return Err(CompilerError::SemanticError("Global variable assignment must be a literal\nConstant folding isn't currently supported")),
                    }
                } else {
                    let t = expr.verify(context)?;
                    if t != self.type_of {
                        return Err(CompilerError::SemanticError("Declaration type must match"));
                    }
                }
            },

            (false, Some(DeclarationValue::Function(stmts))) => {
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
            },

            (true, Some(_)) => return Err(CompilerError::SemanticError("Can't define extern ")),
        };

        Ok(())
    }
}

