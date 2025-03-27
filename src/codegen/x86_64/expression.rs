use crate::{ast::{expressions::BinaryExpr, Expression, Type}, codegen::{error::CodegenError, x86_64::registers::Register}};

use super::{helpers::get_size, instance::{GeneratorInstance, Scratch}, instructions::Instr, registers::{RegisterSize, SizedRegister, ARG_REGS}};

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

            Expression::Ternary(expr) => {
                let condition = self.gen_expr(&expr.condition)?;

                let to_branch = self.new_label();
                let to_end = self.new_label();
                // TODO: Get size
                let result = self.alloc_scratch(RegisterSize::DWord)?;

                let cmp = Instr::Cmp(condition.reg.to_string(), "0".to_string());
                let je = Instr::Je(to_branch);
                let jmp = Instr::Jmp(to_end);

                self.add_instr(cmp);
                self.add_instr(je);
                let if_t = self.gen_expr(&expr.true_expr)?;
                self.add_instr(Instr::Mov(
                        result.reg.to_string(),
                        if_t.reg.to_string()));
                self.add_instr(jmp);
                self.add_label(to_branch);
                let if_f = self.gen_expr(&expr.false_expr)?;
                self.add_instr(Instr::Mov(
                        result.reg.to_string(),
                        if_f.reg.to_string()));
                self.add_label(to_end);

                Ok(result)
            },

            Expression::LogicalOr(expr) => {
                let (a, b) = self.get_binary_scratches(expr)?;
                // TODO: Get size
                let result = self.alloc_scratch(a.reg.size)?;

                let to_true = self.new_label();
                let to_false = self.new_label();
                let to_end = self.new_label();

                let cmp_a = Instr::Cmp(a.reg.to_string(), "0".to_string());
                let cmp_b = Instr::Cmp(b.reg.to_string(), "0".to_string());
                let j_if_a_true = Instr::Jne(to_true);
                let j_if_b_false = Instr::Je(to_false);
                let j_to_end = Instr::Jmp(to_end);
                let ret_true = Instr::Mov(result.reg.to_string(), "1".to_string());
                let ret_false = Instr::Mov(result.reg.to_string(), "0".to_string());

                self.add_instr(cmp_a);
                self.add_instr(j_if_a_true);
                self.add_instr(cmp_b);
                self.add_instr(j_if_b_false);

                self.add_label(to_true);
                self.add_instr(ret_true);
                self.add_instr(j_to_end);

                self.add_label(to_false);
                self.add_instr(ret_false);
    
                self.add_label(to_end);

                Ok(result)
            },

            Expression::LogicalAnd(expr) => {
                let (a, b) = self.get_binary_scratches(expr)?;
                // TODO: Get size
                let result = self.alloc_scratch(a.reg.size)?;

                let to_false = self.new_label();
                let to_end = self.new_label();

                let cmp_a = Instr::Cmp(a.reg.to_string(), "0".to_string());
                let cmp_b = Instr::Cmp(b.reg.to_string(), "0".to_string());
                let j_if_false = Instr::Je(to_false);
                let j_to_end = Instr::Jmp(to_end);
                let ret_true = Instr::Mov(result.reg.to_string(), "1".to_string());
                let ret_false = Instr::Mov(result.reg.to_string(), "0".to_string());

                self.add_instr(cmp_a);
                self.add_instr(j_if_false.clone());
                self.add_instr(cmp_b);
                self.add_instr(j_if_false);
                self.add_instr(ret_true);
                self.add_instr(j_to_end);

                self.add_label(to_false);
                self.add_instr(ret_false);
    
                self.add_label(to_end);

                Ok(result)
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

            Expression::ArrayIndex(args) => {
                let (arr, ind) = self.get_binary_scratches(args)?;

                /* TODO: Figure out how to get the size (answer: store type in
                 * exprs)
                let size = ??

                let instr = Instr::Mov(
                    arr.reg.to_string()
                    format!("[{} + {}*{}]", arr, ind, */

                todo!()
            }

            Expression::FuncCall(expr) => {
                let fn_name = match &expr.func {
                    Expression::Identifier(s) => s,
                    _ => panic!("Function needs to be identifier"),
                };

                let (fn_symbol, fn_type) = self.get_symbol(fn_name)
                    .expect("Undefined").clone();

                let ret_type = match fn_type {
                    Type::Function(f) => f.return_type,
                    _ => panic!("Function type needs to be function!"),
                };

                let is_void = match ret_type {
                    Type::Void => true,
                    _ => false,
                };


                for (i, arg) in expr.args.iter().enumerate() {
                    if i >= 6 { //Reached end of register args
                        break;
                    }

                    let arg_scratch = self.gen_expr(arg)?;

                    let arg_reg = SizedRegister {
                        reg: ARG_REGS[i],
                        size: arg_scratch.reg.size
                    };

                    let mov = Instr::Mov(
                        arg_reg.to_string(), 
                        arg_scratch.reg.to_string());

                    self.add_instr(mov);
                }

                // Push stack args on
                let num_args = expr.args.len();
                if num_args > 6 {
                    for i in (6..num_args).rev() { // Go backwards per ABI
                        let mut arg_scratch = self.gen_expr(&expr.args[i])?;

                        // Stack pushes need to be QWords (idk why)
                        arg_scratch.reg.size = RegisterSize::QWord;

                        self.add_instr(Instr::Push(arg_scratch.reg.to_string()));
                    }
                }

                self.add_instr(Instr::Call(fn_symbol));

                let bytes_to_pop = (num_args - 6) * 8;
                if bytes_to_pop > 0 {
                    let instr = Instr::Add(
                        "RSP".to_string(),
                        bytes_to_pop.to_string());
                    self.add_instr(instr);
                }

                let ret = self.alloc_scratch(get_size(&ret_type))?;
                if !is_void {
                    let rax = SizedRegister { reg: Register::Rax, size: ret.reg.size };
                    let mov_ret = Instr::Mov(ret.reg.to_string(), rax.to_string());
                    self.add_instr(mov_ret);
                } 

                Ok(ret)
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
                // TODO: Use correct size
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
