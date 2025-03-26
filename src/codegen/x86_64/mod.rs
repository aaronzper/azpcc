mod registers;
mod instance;
mod declaration;
mod statement;
mod expression;

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

        let mut instance = GeneratorInstance::new();

        for decl in &trans_unit.declarations {
            instance.gen_declaration(decl)?;
        }

        Ok(instance.get_instructions())
    }

    fn assemble(&self, assembly: &[String], options: &AssemblerOptions) ->
        Result<(), CodegenError> {
        
        todo!()
    }
}
