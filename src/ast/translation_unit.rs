use super::{Declaration, SemanticUnit};

#[derive(Debug)]
pub struct TranslationUnit {
    pub declarations: Box<[Declaration]>,
}

impl SemanticUnit for TranslationUnit {
    fn verify_with_context(&self, context: &mut super::Context) -> 
            Result<(), crate::error::CompilerError> {

        for decl in &self.declarations {
            decl.verify_with_context(context)?;
        }
        
        Ok(())
    }
}
