
use crate::{ast::{declaration::DeclarationValue, Declaration}, codegen::error::CodegenError};

use super::{instance::GeneratorInstance, instructions::Instr};

impl GeneratorInstance {
    pub fn gen_declaration(&mut self, decl: &Declaration) -> 
        Result<(), CodegenError> {
        
        let symbol = decl.name.clone();

        match &decl.value {
            None => self.add_extern(symbol),
            Some(val) => match (self.global_scope(), val)  {
                (true, DeclarationValue::Function(stmts)) => {
                    self.add_fn_symbol(symbol);
                    self.enter_scope();

                    self.add_instr(Instr::Push("RBP".to_string()));
                    self.add_instr(
                        Instr::Mov("RBP".to_string(), "RSP".to_string())
                    );

                    // TODO: set up args

                    for stmt in stmts {
                        self.gen_statement(stmt)?;
                    }

                    self.add_instr(
                        Instr::Mov("RSP".to_string(), "RBP".to_string())
                    );
                    self.add_instr(Instr::Pop("RBP".to_string()));
                    self.add_instr(Instr::Ret);

                    self.exit_scope();
                },

                (true, DeclarationValue::Variable(e)) => {
                    self.add_symbol(symbol.clone(), symbol);
                    // TODO: Finish
                }

                (false, DeclarationValue::Function(_)) => 
                    panic!("Can't define local function!"),

                (false, DeclarationValue::Variable(e)) => todo!()
            }
        }

        Ok(())
    }
}
