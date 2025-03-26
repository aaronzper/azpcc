use crate::{ast::Expression, codegen::error::CodegenError};

use super::{instance::{GeneratorInstance, Scratch}, instructions::Instr, registers::RegisterSize};

impl GeneratorInstance {
    pub fn gen_expr(&mut self, expr: &Expression) -> 
        Result<Scratch, CodegenError> {
        
        match expr {
            Expression::Add(args) => {
                let a = self.gen_expr(&args.first)?;
                let b = self.gen_expr(&args.second)?;

                let instr = Instr::Add(a.reg.to_string(), b.reg.to_string());
                self.add_instr(instr);

                Ok(a)
            },

            Expression::IntLiteral(x) => {
                let scratch = self.alloc_scratch(RegisterSize::Byte)?;
                let instr = Instr::Mov(scratch.reg.to_string(), x.to_string());
                self.add_instr(instr);
                Ok(scratch)
            }

            _ => todo!()
        }
    }
}
