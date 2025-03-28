
use crate::{ast::{declaration::DeclarationValue, Declaration, Expression, Type}, codegen::error::CodegenError};

use super::{helpers::get_size, instance::{GeneratorInstance, ScopeVariable}, instructions::Instr, registers::{SizedRegister, ARG_REGS}};

impl GeneratorInstance {
    pub fn gen_declaration(&mut self, decl: &Declaration) -> 
        Result<(), CodegenError> {
        
        let symbol = decl.name.clone();

        match &decl.value {
            None => {
                if decl.external {
                    self.add_extern(symbol.clone(), decl.type_of.clone());
                } else {
                    if self.global_scope() {
                        self.add_global(symbol.clone(), decl.type_of.clone());

                        match decl.type_of {
                            Type::Function(_) => (),
                            _ => self.add_bss(symbol, &decl.type_of),
                        };
                    } else {
                        self.add_local(symbol, decl.type_of.clone());
                    }
                }
            },

            Some(val) => match (self.global_scope(), val)  {
                (true, DeclarationValue::Function(stmts)) => {
                    self.add_global(symbol.clone(), decl.type_of.clone());
                    self.add_fn_label(symbol);

                    let ret_label = self.new_label();
                    self.return_label = Some(ret_label);
                    let _s = self.enter_scope();

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
                        let symbol = arg_n.clone();
                        let size = get_size(arg_t);

                        let asm_rep = if i < 6 {
                            let reg = SizedRegister {
                                reg: ARG_REGS[i],
                                size,
                            };
                            self.arg_regs.insert(reg.reg);
                            reg.to_string()
                        } else {
                            // Arg 7+ starts at RBP+16
                            let rbp_offset = 16 + ((i-6) * 8);
                            format!("{} [RBP + {}]", size, rbp_offset)
                        };

                        let var = ScopeVariable {
                            asm_rep,
                            type_of: arg_t.to_owned(),
                        };

                        self.add_symbol_with_asm(symbol, var);
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
                    self.arg_regs.clear();
                },

                (true, DeclarationValue::Variable(e)) => {
                    self.add_global(symbol.clone(), decl.type_of.clone());
                    
                    let asm = match e {
                        // TODO: Handle sizes
                        Expression::IntLiteral(i) => format!("dd {}", i),

                        Expression::CharLiteral(c) => format!("db {}", c),

                        Expression::StringLiteral(s) => format!("db {}, 0", s),

                        _ => panic!("Must init global w/ a literal"),
                    };

                    self.add_data(symbol, asm);
                }

                (false, DeclarationValue::Function(_)) => 
                    panic!("Can't define local function!"),

                (false, DeclarationValue::Variable(e)) => {
                    let asm_var = self.add_local(symbol, decl.type_of.clone());
                    let asm_val = self.gen_expr(e)?;

                    self.add_instr(Instr::Mov(asm_var, asm_val.reg.to_string()));
                }
            }
        }

        Ok(())
    }
}
