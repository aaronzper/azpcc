use super::Type;

// TODO: Struct/pointer subfield accessing, +x, float literals
#[derive(Debug)]
pub enum Expression {
    Assignment(Box<BinaryExpr>),// x = y

    Ternary(Box<TernaryExpr>),  // x ? y : z

    LogicalOr(Box<BinaryExpr>), // x || y
    LogicalAnd(Box<BinaryExpr>),// x && y
    BitwiseOr(Box<BinaryExpr>), // x |  y
    BitwiseXor(Box<BinaryExpr>),// x ^  y
    BitwiseAnd(Box<BinaryExpr>),// x &  y

    Equality(Box<BinaryExpr>),  // x == y
    Inequality(Box<BinaryExpr>),// x != y

    LTCompare(Box<BinaryExpr>), // x <  y
    GTCompare(Box<BinaryExpr>), // x >  y
    LECompare(Box<BinaryExpr>), // x <= y
    GECompare(Box<BinaryExpr>), // x >= y
    
    ShiftLeft(Box<BinaryExpr>), // x << y
    ShiftRight(Box<BinaryExpr>),// x >> y
    
    Add(Box<BinaryExpr>),       // x + y
    Subtract(Box<BinaryExpr>),  // x - y
    
    Multiply(Box<BinaryExpr>),  // x * y
    Divide(Box<BinaryExpr>),    // x / y
    Modulo(Box<BinaryExpr>),    // x % y

    Cast(Box<CastExpr>),        // (int)x

    PreInc(Box<UnaryExpr>),     // ++x
    PreDec(Box<UnaryExpr>),     // --x
    PostInc(Box<UnaryExpr>),    // x++
    PostDec(Box<UnaryExpr>),    // x--
    
    AddressOf(Box<UnaryExpr>),  // &x
    Dereference(Box<UnaryExpr>),// *x
    Negate(Box<UnaryExpr>),     // -x
    BitwiseNot(Box<UnaryExpr>), // ~x
    LogicalNot(Box<UnaryExpr>), // !x

    SizeOf(Box<UnaryExpr>),     // sizeof x

    ArrayIndex(Box<BinaryExpr>),// x[y]
    
    FuncCall(Box<FuncCallExpr>),// x(...)
    
    Identifier(String),         // x

    IntLiteral(u64),            // 123
    CharLiteral(u8),            // 'a'
    StringLiteral(String),      // "hello world\n"
}

#[derive(Debug)]
pub struct TernaryExpr {
    pub condition: Expression,
    pub true_expr: Expression,
    pub false_expr: Expression,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub first: Expression,
    pub second: Expression,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub expr: Expression
}

#[derive(Debug)]
pub struct CastExpr {
    pub cast_to: Type,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct FuncCallExpr {
    pub func: Expression,
    pub args: Box<[Expression]>,
}
