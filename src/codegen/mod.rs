use std::path::Path;

use error::CodegenError;
use target_lexicon::Architecture;
use x86_64::X86_64Generator;

use crate::{ast::TranslationUnit, error::CompilerError};

pub mod triple;
pub mod error;
mod x86_64;

pub struct AssemblerOptions<'a> {
    /// Link to final executable - false corresponds to -c flag
    pub link: bool,

    /// Path to output file
    pub output: &'a Path, 
}

pub trait Generator {
    fn generate(&self, trans_unit: &TranslationUnit) -> 
        Result<String, CodegenError>;

    fn assemble(&self, assembly: &[String], options: &AssemblerOptions) ->
        Result<(), CodegenError>;
}

pub fn get_generator(arch: &Architecture) -> 
    Result<Box<dyn Generator>, CompilerError> {

    match arch {
        Architecture::X86_64 => Ok(Box::new(X86_64Generator::new())),
        _ => Err(CompilerError::NotSupported("Targeting non x86_64")),
    }
}
