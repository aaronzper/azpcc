
use crate::{ast::Declaration, codegen::error::CodegenError};

use super::instance::GeneratorInstance;

impl GeneratorInstance {
    pub fn gen_declaration(&self, decl: &Declaration) -> 
        Result<(), CodegenError> {
        
        Ok(())
    }
}
