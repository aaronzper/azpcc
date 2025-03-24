use crate::error::CompilerError;

use super::{Context, Declaration};

#[derive(Debug)]
pub struct TranslationUnit {
    pub declarations: Box<[Declaration]>,
}

impl TranslationUnit {
    pub fn verify(&self, context: &mut Context) -> Result<(), CompilerError> {
        for decl in &self.declarations {
            decl.verify(context)?;
        }
        
        Ok(())
    }
}


