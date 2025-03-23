use std::path::PathBuf;

#[derive(Debug)]
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
