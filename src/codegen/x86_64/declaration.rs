
use crate::{ast::{declaration::DeclarationValue, Declaration}, codegen::error::CodegenError};

use super::{instance::GeneratorInstance, instructions::Instr};

impl GeneratorInstance {
    pub fn gen_declaration(&mut self, decl: &Declaration) -> 
        Result<(), CodegenError> {
        
        let symbol = decl.name.clone();

        match &decl.value {
            None => {
                let asm_rep = match decl.type_of {
                    crate::ast::Type::Function(_) => symbol.clone(),
                    _ => format!("[{}]", symbol),
                };

                if decl.external {
                    self.add_extern(symbol.clone(), asm_rep);
                } else {
                    self.add_symbol(symbol.clone(), asm_rep);
                }
            },

            Some(val) => match (self.global_scope(), val)  {
                (true, DeclarationValue::Function(stmts)) => {
                    self.add_fn_symbol(symbol);

                    let ret_label = self.new_label();
                    self.return_label = Some(ret_label);
                    self.enter_scope();

                    self.add_instr(Instr::Push("RBP".to_string()));
                    self.add_instr(
                        Instr::Mov("RBP".to_string(), "RSP".to_string())
                    );

                    // TODO: set up args

                    for stmt in stmts {
                        self.gen_statement(stmt)?;
                    }

                    self.add_label(ret_label);
                    self.add_instr(
                        Instr::Mov("RSP".to_string(), "RBP".to_string())
                    );
                    self.add_instr(Instr::Pop("RBP".to_string()));
                    self.add_instr(Instr::Ret);

                    self.return_label = None;
                    self.exit_scope();
                },

                (true, DeclarationValue::Variable(e)) => {
                    self.add_symbol(symbol.clone(), format!("[{}]", symbol));
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
