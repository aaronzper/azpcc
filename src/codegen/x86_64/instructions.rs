use std::fmt::Display;

pub enum Instr {
    Mov(String, String),

    Add(String, String),

    Push(String),
    Pop(String),

    Jmp(u64),

    Ret,
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Mov(a, b) => write!(f, "mov {}, {}", a, b),

            Instr::Add(a, b) => write!(f, "add {}, {}", a, b),

            Instr::Push(a) => write!(f, "push {}", a),
            Instr::Pop(a) => write!(f, "pop {}", a),

            Instr::Jmp(a) => write!(f, "jmp .L{}", a),

            Instr::Ret => write!(f, "ret"),
        }
    }
}
