use std::path::Path;

use target_lexicon::Triple;

use crate::error::CompilerError;

pub mod triple;
mod x86_64;

pub fn generate(triple: Triple, output: &Path) -> Result<(), CompilerError> {
    match triple.architecture {
        target_lexicon::Architecture::X86_64 => x86_64::generate(triple, output),
        _ => Err(CompilerError::NotSupported("Compiling other than x86-64")),
    }
}
