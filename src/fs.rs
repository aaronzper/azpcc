use std::{fs::File, io::Read, path::Path};

use log::trace;

use crate::error::CompilerError;

pub fn read_file(path: &Path) -> Result<String, CompilerError> {
    trace!("Reading file: {:?}", path);
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
