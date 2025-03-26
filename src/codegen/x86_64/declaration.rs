
use crate::{ast::Declaration, codegen::error::CodegenError};

use super::instance::GeneratorInstance;

impl GeneratorInstance {
    pub fn gen_declaration(&mut self, decl: &Declaration) -> 
        Result<(), CodegenError> {
        
        let symbol = decl.name.clone();

        match decl.value {
            None => self.add_extern(symbol),
            Some(_) => self.add_symbol(symbol.clone(), symbol),
        }

        Ok(())
    }
}
