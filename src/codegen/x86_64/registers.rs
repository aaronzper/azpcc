use std::fmt::Display;

use crate::error::CompilerError;

/// Represents an unsized, x86-64 general-purpose register
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Register {
    Rax = 0,
    Rbx = 1,
    Rcx = 2,
    Rdx = 3,
    Rsi = 4,
    Rdi = 5,
    R8  = 6,
    R9  = 7,
    R10 = 8,
    R11 = 9,
    R12 = 10,
    R13 = 11,
    R14 = 12,
    R15 = 13,
}

pub const NUM_REGS: u8 = 14;

/// The registers used as function args, in the order that arguments are passed
pub const ARG_REGS: [Register; 6] = [ 
    Register::Rdi,
    Register::Rsi,
    Register::Rdx,
    Register::Rcx,
    Register::R8,
    Register::R9,
];

impl TryFrom<u8> for Register {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= NUM_REGS {
            return Err("Register number must be 0-13");
        }

        // This is safe cause we checked above that its legit
        Ok(unsafe {
            std::mem::transmute(value)
        })
    }
}

/// Represents a potential register size in x86
#[derive(Copy, Clone)]
pub enum RegisterSize {
    QWord,// 64bit
    DWord,// 32bit
    Word, // 16bit
    Byte, // 8bit
}

pub struct SizedRegister {
    pub reg: Register,
    pub size: RegisterSize,
}

impl Display for SizedRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The "core" of the name (_a_ or r8_)
        let core = match self.reg {
            Register::Rax => "A",
            Register::Rbx => "B",
            Register::Rcx => "C",
            Register::Rdx => "D",
            Register::Rsi => "SI",
            Register::Rdi => "DI",
            Register::R8  => "R8",
            Register::R9  => "R9",
            Register::R10 => "R10",
            Register::R11 => "R11",
            Register::R12 => "R12",
            Register::R13 => "R13",
            Register::R14 => "R14",
            Register::R15 => "R15",
        };

        if self.reg as u8 >= 6 { // R8 and above
            match self.size {
                RegisterSize::QWord => write!(f, "{}", core),
                RegisterSize::DWord => write!(f, "{}D", core),
                RegisterSize::Word => write!(f, "{}W", core),
                RegisterSize::Byte => write!(f, "{}B", core),
            }
        } else if self.reg as u8 >= 4 { // RSI and RDI
            match self.size {
                RegisterSize::QWord => write!(f, "R{}", core),
                RegisterSize::DWord => write!(f, "E{}", core),
                RegisterSize::Word => write!(f, "{}", core),
                RegisterSize::Byte => write!(f, "{}L", core),
            }
        } else {
            match self.size {
                RegisterSize::QWord => write!(f, "R{}X", core),
                RegisterSize::DWord => write!(f, "E{}X", core),
                RegisterSize::Word => write!(f, "{}X", core),
                RegisterSize::Byte => write!(f, "{}L", core),
            }
        }
    }
}
