use crate::{ast::Statement, codegen::error::CodegenError};

use super::{instance::GeneratorInstance, instructions::Instr, registers::{Register, SizedRegister}};

impl GeneratorInstance {
    pub fn gen_statement(&mut self, stmt: &Statement) ->
        Result<(), CodegenError> {
    
        match stmt {
            Statement::Compound(stmts) => {
                self.enter_scope();
                    
                for s in stmts {
                    self.gen_statement(s)?;
                }

                self.exit_scope();
            },

            Statement::Declaration(decl) => self.gen_declaration(decl)?,

            Statement::Expression(expr) => { self.gen_expr(&expr)?; },

            Statement::If(if_stmt) => todo!(),

            Statement::WhileLoop(while_stmt) => todo!(),

            Statement::Return(ret) => {
                if let Some(expr) = &**ret {
                    let ret_val = self.gen_expr(&expr)?;
                    let rax_sized = SizedRegister {
                        reg: Register::Rax,
                        size: ret_val.reg.size
                    };

                    self.add_instr(
                        Instr::Mov(rax_sized.to_string(), ret_val.reg.to_string())
                    );
                }

                self.add_instr(Instr::Jmp(self.return_label.unwrap()));
            },
        };

        Ok(())
    }
}
