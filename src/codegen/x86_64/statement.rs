use crate::{ast::Statement, codegen::error::CodegenError};

use super::instance::GeneratorInstance;

impl GeneratorInstance {
    pub fn gen_statement(&mut self, stmt: &Statement) ->
        Result<(), CodegenError> {
    
        todo!()
    }
}
