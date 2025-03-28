
use crate::{ast::{declaration::DeclarationValue, Declaration, Type}, codegen::error::CodegenError};

use super::{helpers::get_size, instance::GeneratorInstance, instructions::Instr, registers::{SizedRegister, ARG_REGS}};

impl GeneratorInstance {
    pub fn gen_declaration(&mut self, decl: &Declaration) -> 
        Result<(), CodegenError> {
        
        let symbol = decl.name.clone();

        match &decl.value {
            None => {
                if decl.external {
                    self.add_extern(symbol.clone(), decl.type_of.clone());
                } else {
                    self.add_symbol(symbol.clone(), decl.type_of.clone());

                    match decl.type_of {
                        Type::Function(_) => (),
                        _ => self.add_bss(symbol, &decl.type_of),
                    };
                }
            },

            Some(val) => match (self.global_scope(), val)  {
                (true, DeclarationValue::Function(stmts)) => {
                    self.add_symbol(symbol.clone(), decl.type_of.clone());
                    self.add_fn_label(symbol);

                    let ret_label = self.new_label();
                    self.return_label = Some(ret_label);
                    self.enter_scope();

                    self.add_instr(Instr::Push("RBP".to_string()));
                    self.add_instr(
                        Instr::Mov("RBP".to_string(), "RSP".to_string())
                    );

                    let args = if let Type::Function(ftype) = &decl.type_of {
                        &ftype.args
                    } else {
                        panic!("Function decl type must be func");
                    };

                    for (i, (arg_n, arg_t)) in args.iter().enumerate() {
                        if i >= 6 {
                            todo!("Functions defs with more that 6 args");
                        }

                        let symbol = arg_n.clone();
                        let reg = SizedRegister {
                            reg: ARG_REGS[i],
                            size: get_size(arg_t),
                        }.to_string();

                        self.add_symbol_with_asm(symbol, arg_t.clone(), reg);
                    }

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
                    self.add_symbol(symbol.clone(), decl.type_of.clone());
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
