mod registers;
mod scratch;
mod instance;

use instance::GeneratorInstance;
use crate::ast::TranslationUnit;
use super::{error::CodegenError, AssemblerOptions, Generator};

pub struct X86_64Generator;

impl X86_64Generator {
    pub fn new() -> X86_64Generator {
        X86_64Generator {}
    }
}

impl Generator for X86_64Generator {
    fn generate(&self, trans_unit: &TranslationUnit) -> 
        Result<String, CodegenError> {

        let instance = GeneratorInstance::new();

        todo!()
    }

    fn assemble(&self, assembly: &[String], options: &AssemblerOptions) ->
        Result<(), CodegenError> {
        
        todo!()
    }
}
