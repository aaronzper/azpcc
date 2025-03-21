use target_lexicon::{Architecture, Triple};

pub fn get_triple() -> Triple {
    let mut triple = Triple::host();
    triple.architecture = Architecture::X86_64;

    triple
}
