use std::fmt::Display;

pub enum Instr {
    Mov(String, String),
    Movsx(String, String),
    Lea(String, String),

    Add(String, String),
    Sub(String, String),
    Imul(String, String),
    Idiv(String),
    Neg(String),

    Not(String),

    Push(String),
    Pop(String),

    Jmp(u64),

    Cqo,

    Ret,
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Mov(a, b) => write!(f, "mov {}, {}", a, b),
            Instr::Movsx(a, b) => write!(f, "movsx {}, {}", a, b),
            Instr::Lea(a, b) => write!(f, "lea {}, {}", a, b),

            Instr::Add(a, b) => write!(f, "add {}, {}", a, b),
            Instr::Sub(a, b) => write!(f, "sub {}, {}", a, b),
            Instr::Imul(a, b) => write!(f, "imul {}, {}", a, b),
            Instr::Idiv(a) => write!(f, "idiv {}", a),
            Instr::Neg(a) => write!(f, "neg {}", a),

            Instr::Not(a) => write!(f, "not {}", a),

            Instr::Push(a) => write!(f, "push {}", a),
            Instr::Pop(a) => write!(f, "pop {}", a),

            Instr::Jmp(a) => write!(f, "jmp .L{}", a),

            Instr::Cqo => write!(f, "cqo"),

            Instr::Ret => write!(f, "ret"),
        }
    }
}
