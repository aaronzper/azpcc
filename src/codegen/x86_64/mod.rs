use std::path::Path;
use target_lexicon::Triple;

use crate::error::CompilerError;

pub fn generate(_triple: Triple, _output: &Path) -> Result<(), CompilerError> {
    println!("Code!");
    Ok(())
}
