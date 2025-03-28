use std::{cell::{Cell, RefCell}, collections::{HashMap, HashSet}, rc::Rc};

use log::trace;

use crate::{ast::Type, codegen::{error::CodegenError, x86_64::helpers::get_global_asm}};

use super::{helpers::{get_bytes, get_size}, instructions::Instr, registers::{Register, RegisterSize, SizedRegister, ARG_REGS, NUM_REGS}};

#[derive(Debug, Clone)]
pub struct ScopeVariable {
    pub asm_rep: String,
    pub type_of: Type,
}

pub struct GeneratorInstance {
    /// Tracks which registers are in used as scratch
    scratches: Rc<RefCell<HashMap<Register, bool>>>,

    /// Used for coming up with .L# labels
    label_counter: u64,

    /// Stack of scopes, maps symbol name to asm code for it. 0 is global, 1 is
    /// function scope, 2 is some scope inside that, etc
    scopes: Rc<RefCell<Vec<HashMap<String, ScopeVariable>>>>,

    /// Tracks which argument registers are in use by the current fn
    pub arg_regs: HashSet<Register>,

    /// The label to jump to to return, if we're in a fn
    pub return_label: Option<u64>,

    /// How many bytes below RDP we've allocated to local variables
    rdp_offset: Rc<Cell<usize>>,

    /// External symbols we'll link to later
    externs: Vec<String>,

    /// Global symbols that are linkable by others
    globals: Vec<String>,

    /// Contents of the data section
    data: String,

    /// Contents of the BSS section
    bss: String,

    /// The actual instructions we're making (contents of the text section)
    instructions: String,
}

impl GeneratorInstance {
    pub fn new() -> GeneratorInstance {
        let mut scratches = HashMap::new();
        
        for i in 0..NUM_REGS {
            let reg = i.try_into().unwrap();

            // Don't use non-callee-saved regs for now (args, Rax, R10, R11)
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
            scopes: Rc::new(RefCell::new(vec![HashMap::new()])),
            arg_regs: HashSet::new(),
            return_label: None,
            rdp_offset: Rc::new(Cell::new(0)),
            externs: vec![],
            globals: vec![],
            data: String::new(),
            bss: String::new(),
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
        self.instructions.push_str(&format!(".L{}:", id));
    }

    pub fn add_fn_label(&mut self, label: String) {
        self.instructions.push_str(&format!("{}:\n", label));
    }

    pub fn get_symbol(&self, symbol: &str) -> Option<ScopeVariable> {
        for scope in self.scopes.borrow().iter().rev() {
            match scope.get(symbol) {
                Some(s) => return Some(s.clone()),
                None => continue,
            }
        }

        None
    }

    pub fn global_scope(&self) -> bool { self.scopes.borrow().len() == 1 }

    pub fn enter_scope(&mut self) -> ScopeOwner { 
        ScopeOwner::new(self)
    }

    pub fn add_local(&mut self, symbol: String, type_of: Type) -> String {
        if self.global_scope() {
            panic!("Can't add local if you're global!!");
        }

        let size = get_bytes(&type_of);
        let new_offset = self.rdp_offset.get() + size;

        self.rdp_offset.set(new_offset);
        self.add_instr(Instr::Sub("RSP".to_string(), size.to_string()));

        let asm_rep = format!("{} [RBP - {}]", get_size(&type_of), new_offset);

        self.add_symbol_with_asm(symbol, ScopeVariable { 
            asm_rep: asm_rep.clone(), type_of, });

        asm_rep
    }

    pub fn add_global(&mut self, symbol: String, type_of: Type) {
        let asm_rep = get_global_asm(&symbol, &type_of);
        self.add_symbol_with_asm(symbol, ScopeVariable { asm_rep, type_of });
    }

    pub fn add_symbol_with_asm(&mut self, symbol: String, var: ScopeVariable) {
        if self.global_scope() {
            self.globals.push(symbol.clone());
        }

        trace!("Adding symbol {} as {:?}", symbol, var);

        // unwrap is allowed cause we should always have at least 1
        self.scopes.borrow_mut().last_mut().unwrap().insert(symbol, var);
    }

    pub fn add_extern(&mut self, symbol: String, type_of: Type) {
        assert!(self.global_scope());

        self.externs.push(symbol.clone());

        let asm_rep = get_global_asm(&symbol, &type_of);
        self.scopes.borrow_mut().last_mut().unwrap().insert(symbol, 
            ScopeVariable { asm_rep, type_of, });
    }
    
    pub fn add_instr(&mut self, instr: Instr) {
        self.instructions.push_str(&format!("\t{}\n", instr));
    }

    pub fn add_bss(&mut self, symbol: String, type_of: &Type) {
        let size = get_bytes(type_of);
        self.bss.push_str(&format!("{}: resb {}\n", symbol, size));
    }

    pub fn add_data(&mut self, symbol: String, asm: String) {
        self.data.push_str(&format!("{}: {}\n", symbol, asm));
    }

    pub fn get_instructions(&self) -> String {
        let mut asm = String::from("BITS 64\nDEFAULT REL\n\n");

        for e in &self.externs {
            asm.push_str(&format!("EXTERN {}\n", e));
        }

        for g in &self.globals {
            asm.push_str(&format!("GLOBAL {}\n", g));
        }

        asm.push_str("\nSECTION .data\n");
        asm.push_str(&self.data);

        asm.push_str("\nSECTION .bss\n");
        asm.push_str(&self.bss);
        
        asm.push_str("\nSECTION .text\n");
        asm.push_str(&self.instructions);

        asm
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

/// "Owns" a scope that get cleaned up after
pub struct ScopeOwner {
    scopes: Rc<RefCell<Vec<HashMap<String, ScopeVariable>>>>,
    rdp_offset: Rc<Cell<usize>>,
    old_offset: usize,
}

impl ScopeOwner {
    fn new(instance: &mut GeneratorInstance) -> ScopeOwner {
        let so = ScopeOwner {
            scopes: instance.scopes.clone(),
            rdp_offset: instance.rdp_offset.clone(),
            old_offset: instance.rdp_offset.get(),
        };
        so.scopes.borrow_mut().push(HashMap::new());
        so
    }
}

impl Drop for ScopeOwner {
    fn drop(&mut self) {
        self.rdp_offset.set(self.old_offset);
        let dropped = self.scopes.borrow_mut().pop();
        trace!("Dropping scope {:#?}", dropped);
    }
}
