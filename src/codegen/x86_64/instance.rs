use std::{cell::RefCell, collections::HashMap, rc::Rc};

use log::trace;

use crate::codegen::error::CodegenError;

use super::{instructions::Instr, registers::{Register, RegisterSize, SizedRegister, ARG_REGS, NUM_REGS}};

pub struct GeneratorInstance {
    /// Tracks which registers are in used as scratch
    scratches: Rc<RefCell<HashMap<Register, bool>>>,

    /// Used for coming up with .L# labels
    label_counter: u64,

    /// Stack of scopes, maps symbol name to asm code for it. 0 is global, 1 is
    /// function scope, 2 is some scope inside that, etc
    scopes: Vec<HashMap<String, String>>,

    /// External symbols we'll link to later
    externs: Vec<String>,

    /// Global symbols that are linkable by others
    globals: Vec<String>,

    /// The label to jump to to return, if we're in a fn
    pub return_label: Option<u64>,

    /// The actual instructions we're making
    instructions: String,
}

impl GeneratorInstance {
    pub fn new() -> GeneratorInstance {
        let mut scratches = HashMap::new();
        
        for i in 0..NUM_REGS {
            let reg = i.try_into().unwrap();

            // Don't use non-callee-saved regs for now (args, Rax, R10, R11)
            // TODO: Change
            let used = ARG_REGS.contains(&reg) 
                || reg == Register::Rax 
                || reg == Register::R10 
                || reg == Register::R11;

            if !used { 
                scratches.insert(reg, false);
            }
        }

        GeneratorInstance {
            scratches: Rc::new(RefCell::new(scratches)),
            label_counter: 0,
            scopes: vec![HashMap::new()],
            externs: vec![],
            globals: vec![],
            return_label: None,
            instructions: String::new(),
        }
    }

    pub fn alloc_scratch<'a>(&'a self, size: RegisterSize) -> 
        Result<Scratch, CodegenError> {

        let mut scratches = self.scratches.borrow_mut();
        
        let reg = match scratches.iter().find(|(_, taken)| !*taken ) {
            Some((reg, _)) => reg.to_owned(),
            None => return Err(CodegenError::OutOfScratch),
        };
        
        scratches.insert(reg, true);

        Ok(Scratch {
            reg: SizedRegister { reg, size },
            scratches: self.scratches.clone(),
        })
    }

    pub fn new_label(&mut self) -> u64 {
        let id = self.label_counter;
        self.label_counter += 1;
        id
    }

    pub fn add_label(&mut self, id: u64) {
        let label = format!(".L{}", id);
        self.instructions.push_str(&format!("{}:", label));
    }

    pub fn get_symbol_asm(&self, symbol: &str) -> Option<&str> {
        for scope in self.scopes.iter().rev() {
            match scope.get(symbol) {
                Some(s) => return Some(s),
                None => continue,
            }
        }

        None
    }

    pub fn global_scope(&self) -> bool { self.scopes.len() == 1 }

    pub fn enter_scope(&mut self) { self.scopes.push(HashMap::new()); }

    pub fn exit_scope(&mut self) { self.scopes.pop(); }

    pub fn add_fn_symbol(&mut self, symbol: String) {
        self.add_symbol(symbol.clone(), symbol.clone());
        self.instructions.push_str(&format!("{}:\n", symbol));
    }

    pub fn add_symbol(&mut self, symbol: String, asm: String) {
        let is_global = self.global_scope();

        if is_global {
            self.globals.push(symbol.clone());
        }

        trace!("Adding symbol {} as {}", symbol, asm);

        // unwrap is allowed cause we should always have at least 1
        self.scopes.last_mut().unwrap().insert(symbol, asm);
    }

    pub fn add_extern(&mut self, symbol: String, asm: String) {
        assert!(self.global_scope());
        self.externs.push(symbol.clone());
        self.scopes.last_mut().unwrap().insert(symbol, asm);
    }
    
    pub fn add_instr(&mut self, instr: Instr) {
        self.instructions.push_str(&format!("\t{}\n", instr));
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
pub struct Scratch {
    pub reg: SizedRegister,
    scratches: Rc<RefCell<HashMap<Register, bool>>>,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let mut scratches = self.scratches.borrow_mut();
        scratches.insert(self.reg.reg, false);
    }
}
