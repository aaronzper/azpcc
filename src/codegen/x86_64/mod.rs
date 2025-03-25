use crate::ast::TranslationUnit;

use super::{AssemblerOptions, Generator};

mod registers;

pub struct X86_64Generator {

}

impl X86_64Generator {
    pub fn new() -> X86_64Generator {
        X86_64Generator {}
    }
}

impl Generator for X86_64Generator {
    fn generate(&self, trans_unit: &TranslationUnit) -> 
        Result<String, crate::error::CompilerError> {

        todo!()
    }

    fn assemble(&self, assembly: &[String], options: &AssemblerOptions) ->
        Result<(), crate::error::CompilerError> {
        
        todo!()
    }
}
