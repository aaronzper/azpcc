use std::fmt::Display;

#[derive(Clone)]
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
    Or(String, String),
    Xor(String, String),
    And(String, String),

    Push(String),
    Pop(String),

    Call(String),
    Jmp(u64),

    Cmp(String, String),
    Je(u64),
    Jne(u64),
    Jl(u64),
    Jg(u64),
    Jle(u64),
    Jge(u64),

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
            Instr::Or(a, b) => write!(f, "or {}, {}", a, b),
            Instr::Xor(a, b) => write!(f, "xor {}, {}", a, b),
            Instr::And(a, b) => write!(f, "and {}, {}", a, b),

            Instr::Push(a) => write!(f, "push {}", a),
            Instr::Pop(a) => write!(f, "pop {}", a),

            Instr::Call(a) => write!(f, "call {}", a),
            Instr::Jmp(a) => write!(f, "jmp .L{}", a),

            Instr::Cmp(a, b) => write!(f, "cmp {}, {}", a, b),
            Instr::Je(a) => write!(f, "je .L{}", a),
            Instr::Jne(a) => write!(f, "jne .L{}", a),
            Instr::Jl(a) => write!(f, "jl .L{}", a),
            Instr::Jg(a) => write!(f, "jg .L{}", a),
            Instr::Jle(a) => write!(f, "jle .L{}", a),
            Instr::Jge(a) => write!(f, "jge .L{}", a),

            Instr::Cqo => write!(f, "cqo"),

            Instr::Ret => write!(f, "ret"),
        }
    }
}
