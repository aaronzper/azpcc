use std::path::Path;

use super::Declaration;

#[derive(Debug)]
pub struct TranslationUnit<'a> {
    pub declarations: Box<[Declaration]>,
    pub file: &'a Path, // Might just make a PathBuf
}
