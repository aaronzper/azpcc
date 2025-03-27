use crate::{ast::{expressions::BinaryExpr, Expression}, codegen::{error::CodegenError, x86_64::registers::Register}};

use super::{helpers::get_size, instance::{GeneratorInstance, Scratch}, instructions::Instr, registers::{RegisterSize, SizedRegister}};

impl GeneratorInstance {
    fn get_binary_scratches(&mut self, args: &BinaryExpr) -> 
        Result<(Scratch, Scratch), CodegenError> {
        let a = self.gen_expr(&args.first)?;
        let b = self.gen_expr(&args.second)?;

        Ok((a,b))
    }
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
                let (a, b) = self.get_binary_scratches(args)?;

                let instr = Instr::Add(a.reg.to_string(), b.reg.to_string());
                self.add_instr(instr);

                Ok(a)
            },

            Expression::Subtract(args) => {
                let (a, b) = self.get_binary_scratches(args)?;

                let instr = Instr::Sub(a.reg.to_string(), b.reg.to_string());
                self.add_instr(instr);

                Ok(a)
            },

            Expression::Multiply(args) => {
                let (a, b) = self.get_binary_scratches(args)?;

                let instr = Instr::Imul(a.reg.to_string(), b.reg.to_string());
                self.add_instr(instr);

                Ok(a)
            },

            Expression::Divide(args) => {
                let (a, b) = self.get_binary_scratches(args)?;

                let rax = SizedRegister { reg: Register::Rax, size: a.reg.size }
                    .to_string();

                let instrs = [
                    Instr::Push("RDX".to_string()),
                    Instr::Mov(rax.clone(), a.reg.to_string()),
                    Instr::Cqo,
                    Instr::Idiv(b.reg.to_string()),
                    Instr::Mov(a.reg.to_string(), rax),
                    Instr::Pop("RDX".to_string()),
                ];

                for instr in instrs {
                    self.add_instr(instr);
                }

                Ok(a)
            },
            
            Expression::Modulo(args) => {
                let (a, b) = self.get_binary_scratches(args)?;

                let rax = SizedRegister { reg: Register::Rax, size: a.reg.size }
                    .to_string();
                
                let rdx = SizedRegister { reg: Register::Rdx, size: a.reg.size }
                    .to_string();

                let instrs = [
                    Instr::Push("RDX".to_string()),
                    Instr::Mov(rax.clone(), a.reg.to_string()),
                    Instr::Cqo,
                    Instr::Idiv(b.reg.to_string()),
                    Instr::Mov(a.reg.to_string(), rdx),
                    Instr::Pop("RDX".to_string()),
                ];

                for instr in instrs {
                    self.add_instr(instr);
                }

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
