use std::fmt::Display;

pub enum Instr {
    Mov(String, String),

    Push(String),
    Pop(String),

    Ret,
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Mov(a, b) => write!(f, "mov {}, {}", a, b),

            Instr::Push(a) => write!(f, "push {}", a),
            Instr::Pop(a) => write!(f, "pop {}", a),

            Instr::Ret => write!(f, "ret"),
        }
    }
}
