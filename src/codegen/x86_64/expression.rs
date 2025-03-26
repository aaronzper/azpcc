use crate::{ast::Expression, codegen::error::CodegenError};

use super::{helpers::get_size, instance::{GeneratorInstance, Scratch}, instructions::Instr, registers::RegisterSize};

impl GeneratorInstance {
    pub fn gen_expr(&mut self, expr: &Expression) -> 
        Result<Scratch, CodegenError> {
        
        match expr {
            Expression::Assignment(x) => {
                if let Expression::Identifier(id) = &x.first {
                    let (symbol, _) = self.get_symbol(&id)
                        .expect("Undefined").to_owned();

                    let scratch = self.gen_expr(&x.second)?;

                    let instr = Instr::Mov(
                        symbol,
                        scratch.reg.to_string());
                    self.add_instr(instr);

                    Ok(scratch)
                } else {
                    // TODO: Support other lvalues:
                    todo!("Can't assign to non identifier right now");
                }
            }

            Expression::Add(args) => {
                let a = self.gen_expr(&args.first)?;
                let b = self.gen_expr(&args.second)?;

                let instr = Instr::Add(a.reg.to_string(), b.reg.to_string());
                self.add_instr(instr);

                Ok(a)
            },

            Expression::Identifier(id) => {
                let (sym, type_of) = self.get_symbol(&id).expect("Undefined");
                let scratch = self.alloc_scratch(get_size(type_of))?;

                let instr = Instr::Mov(
                    scratch.reg.to_string(),
                    sym.to_owned());
                self.add_instr(instr);

                Ok(scratch)
            }

            Expression::IntLiteral(x) => {
                let scratch = self.alloc_scratch(RegisterSize::DWord)?;
                let instr = Instr::Mov(scratch.reg.to_string(), x.to_string());
                self.add_instr(instr);
                Ok(scratch)
            }

            _ => todo!()
        }
    }
}
