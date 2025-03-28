use std::path::PathBuf;

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

    /// Path to output file (otherwise default, e.g. a.out)
    pub output: Option<&'a PathBuf>,
}

pub trait Generator {
    /// Generates assembly code from the given translation unit
    fn generate(&self, trans_unit: &TranslationUnit) -> 
        Result<String, CodegenError>;

    /// Takes a list of path/generated-asm pairs and assembles them. 
    ///
    /// The paths are the original files, paired with their resultant assembly.
    ///
    /// Options also included to tell the assembler whether to link and where to
    /// spit out the output.
    fn assemble(&self, 
        input_pairs: &[(PathBuf, String)],
        options: &AssemblerOptions
    ) -> Result<(), CompilerError>;
}

pub fn get_generator(arch: &Architecture) -> 
    Result<Box<dyn Generator>, CompilerError> {

    match arch {
        Architecture::X86_64 => Ok(Box::new(X86_64Generator::new())),
        _ => Err(CompilerError::NotSupported("Targeting non x86_64")),
    }
}
