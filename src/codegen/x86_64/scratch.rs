use super::{registers::SizedRegister, GeneratorInstance};

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
