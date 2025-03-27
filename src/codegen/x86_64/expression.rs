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

            Expression::Cast(cast) => {
                let mut reg = self.gen_expr(&cast.expr)?;

                let old = reg.reg.to_string();
                let old_sz = reg.reg.size as u8;
                reg.reg.size = get_size(&cast.cast_to);
                let new = reg.reg.to_string();
                let new_sz = reg.reg.size as u8;

                if new_sz > old_sz {
                    let instr = Instr::Movsx(new, old);
                    self.add_instr(instr);
                }

                Ok(reg)
            }

            // TODO: Pre/Post Inc/Dec
            
            Expression::AddressOf(expr) => {
                match &expr.expr {
                    Expression::Identifier(x) => {
                        let reg = self.alloc_scratch(RegisterSize::QWord)?;
                        let (x_asm, _) = self.get_symbol(&x).expect("Undefined");
                        let instr = Instr::Lea(
                            reg.reg.to_string(), 
                            x_asm.to_string());
                        self.add_instr(instr);
                        Ok(reg)
                    },

                    // TODO
                    Expression::Dereference(_) | Expression::ArrayIndex(_) => 
                        todo!(),

                    _ => panic!("Address arg must be lvalue"),
                }
            }

            Expression::Dereference(expr) => {
                let a = self.gen_expr(&expr.expr)?;

                let instr = Instr::Mov(a.reg.to_string(), format!("[{}]", a.reg));
                self.add_instr(instr);

                Ok(a)
            }

            Expression::Negate(expr) => {
                let a = self.gen_expr(&expr.expr)?;

                let instr = Instr::Neg(a.reg.to_string());
                self.add_instr(instr);

                Ok(a)
            }

            Expression::BitwiseNot(expr) => {
                let a = self.gen_expr(&expr.expr)?;

                let instr = Instr::Not(a.reg.to_string());
                self.add_instr(instr);

                Ok(a)
            }

            Expression::LogicalNot(expr) => {
                let a = self.gen_expr(&expr.expr)?;

                let to_branch = self.new_label();
                let to_end = self.new_label();

                let cmp = Instr::Cmp(a.reg.to_string(), "0".to_string());
                let je = Instr::Je(to_branch);
                let mov_0 = Instr::Mov(a.reg.to_string(), "0".to_string());
                let jmp = Instr::Jmp(to_end);
                let mov_1 = Instr::Mov(a.reg.to_string(), "1".to_string());

                self.add_instr(cmp);
                self.add_instr(je);

                self.add_instr(mov_0);
                self.add_instr(jmp);

                self.add_label(to_branch);
                self.add_instr(mov_1);

                self.add_label(to_end);

                Ok(a)
            }

            // TODO
            Expression::SizeOf(_) => todo!(),

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

            Expression::CharLiteral(x) => {
                let scratch = self.alloc_scratch(RegisterSize::Byte)?;
                let instr = Instr::Mov(scratch.reg.to_string(), x.to_string());
                self.add_instr(instr);
                Ok(scratch)
            }

            _ => todo!()
        }
    }
}
