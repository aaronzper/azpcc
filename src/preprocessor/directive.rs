use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove. Just put for now so clippy shuts up
pub enum Directive {
    IncludeGlobal(PathBuf),
    IncludeLocal(PathBuf),
    Define(Definition),
    Raw(String),
}

#[derive(Debug)]
pub struct Definition {
    pub identifier: String,
    pub replacement: Option<String>,
}

