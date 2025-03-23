use std::path::Path;

use target_lexicon::Triple;

pub mod triple;
mod x86_64;

pub fn generate(triple: Triple, output: &Path) {
    match triple.architecture {
        target_lexicon::Architecture::X86_64 => x86_64::generate(triple, output),
        _ => println!("Triple {} not supported", triple),
    }
}
