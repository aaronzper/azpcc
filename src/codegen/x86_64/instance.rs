use std::{cell::RefCell, collections::HashMap};

use crate::codegen::error::CodegenError;

use super::registers::{Register, RegisterSize, SizedRegister, ARG_REGS, NUM_REGS};

pub struct GeneratorInstance {
    /// Tracks which registers are in used as scratch
    scratches: RefCell<HashMap<Register, bool>>,

    /// Used for coming up with .L# labels
    label_counter: u64,

    /// Stack of scopes, maps symbol name to asm code for it. 0 is global, 1 is
    /// function scope, 2 is some scope inside that, etc
    scopes: Vec<HashMap<String, String>>,

    /// External symbols we'll link to later
    externs: Vec<String>,

    /// Global symbols that are linkable by others
    globals: Vec<String>,

    /// The actual instructions we're making
    instructions: String,
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
            externs: vec![],
            globals: vec![],
            instructions: String::new(),
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

    fn add_label(&mut self) -> String {
        let id = self.label_counter;
        self.label_counter += 1;

        let label = format!(".L{}", id);
        self.instructions.push_str(&format!("{}:", label));

        label
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

    pub fn get_instructions(&self) -> String {
        let mut instructions = String::from("BITS 64\n\n");

        for e in &self.externs {
            instructions.push_str(&format!("EXTERN {}\n", e));
        }

        for g in &self.globals {
            instructions.push_str(&format!("GLOBAL {}\n", g));
        }
        
        instructions.push_str("\nSECTION .text\n");

        instructions.push_str(&self.instructions);

        instructions
    }
}

/// "Owns" a scratch register
pub struct Scratch<'a> {
    pub reg: SizedRegister,
    pub generator: &'a GeneratorInstance,
}

impl<'a> Drop for Scratch<'a> {
    fn drop(&mut self) {
        let mut scratches = self.generator.scratches.borrow_mut();
        scratches.insert(self.reg.reg, false);
    }
}
