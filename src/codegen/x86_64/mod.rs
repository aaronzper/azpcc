mod registers;
mod scratch;

use std::{cell::RefCell, collections::HashMap};

use registers::{Register, RegisterSize, SizedRegister, NUM_REGS};
use scratch::Scratch;

use crate::{ast::TranslationUnit, error::CompilerError};

use super::{AssemblerOptions, Generator};


pub struct X86_64Generator {
    scratches: RefCell<HashMap<Register, bool>>,
}

impl X86_64Generator {
    pub fn new() -> X86_64Generator {
        let mut scratches = HashMap::new();
        
        for i in 0..NUM_REGS {
            scratches.insert(i.try_into().unwrap(), false);
        }

        X86_64Generator {
            scratches: RefCell::new(scratches),
        }
    }

    fn alloc_scratch<'a>(&'a self, size: RegisterSize) -> 
        Result<Scratch<'a>, CompilerError> {

        let mut scratches = self.scratches.borrow_mut();
        
        let reg = match scratches.iter().find(|(_, taken)| !*taken ) {
            Some((reg, _)) => reg.to_owned(),
            None => panic!("Out of scratch regs") // TODO
        };
        
        scratches.insert(reg, true);

        Ok(Scratch {
            reg: SizedRegister { reg, size },
            generator: self
        })
    }
}

impl Generator for X86_64Generator {
    fn generate(&self, trans_unit: &TranslationUnit) -> 
        Result<String, crate::error::CompilerError> {

        todo!()
    }

    fn assemble(&self, assembly: &[String], options: &AssemblerOptions) ->
        Result<(), crate::error::CompilerError> {
        
        todo!()
    }
}
