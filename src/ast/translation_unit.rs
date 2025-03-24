use super::Declaration;

#[derive(Debug)]
pub struct TranslationUnit {
    pub declarations: Box<[Declaration]>,
}
