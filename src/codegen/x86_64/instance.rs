use std::{cell::RefCell, collections::HashMap};

use crate::codegen::error::CodegenError;

use super::registers::{Register, RegisterSize, SizedRegister, ARG_REGS, NUM_REGS};
use super::scratch::Scratch;

pub struct GeneratorInstance {
    /// Tracks which registers are in used as scratch
    scratches: RefCell<HashMap<Register, bool>>,

    /// Used for coming up with .L# labels
    label_counter: u64,

    /// Stack of scopes, maps symbol name to asm code for it. 0 is global, 1 is
    /// function scope, 2 is some scope inside that, etc
    scopes: Vec<HashMap<String, String>>
}

impl GeneratorInstance {
    pub fn new() -> GeneratorInstance {
        let mut scratches = HashMap::new();
        
        for i in 0..NUM_REGS {
            let reg = i.try_into().unwrap();
            let used = ARG_REGS.contains(&reg);
            if !used { // Don't arg regs for now (TODO: change)
                scratches.insert(reg, false);
            }
        }

        GeneratorInstance {
            scratches: RefCell::new(scratches),
            label_counter: 0,
            scopes: vec![HashMap::new()],
        }
    }

    fn alloc_scratch<'a>(&'a self, size: RegisterSize) -> 
        Result<Scratch<'a>, CodegenError> {

        let mut scratches = self.scratches.borrow_mut();
        
        let reg = match scratches.iter().find(|(_, taken)| !*taken ) {
            Some((reg, _)) => reg.to_owned(),
            None => return Err(CodegenError::OutOfScratch),
        };
        
        scratches.insert(reg, true);

        Ok(Scratch {
            reg: SizedRegister { reg, size },
            generator: self
        })
    }

    fn create_label(&mut self) -> String {
        let label = self.label_counter;
        self.label_counter += 1;

        format!(".L{}", label)
    }

    fn get_symbol_asm(&self, symbol: &str) -> Option<&str> {
        for scope in self.scopes.iter().rev() {
            match scope.get(symbol) {
                Some(s) => return Some(s),
                None => continue,
            }
        }

        None
    }
}

