use crate::error::CompilerError;

use super::{Context, Type};

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

impl TernaryExpr {
    pub fn verify(&self, context: &mut Context) -> Result<Type, CompilerError> {
        let cond_t = self.condition.verify(context)?;

        if !cond_t.is_integer() {
            return Err(CompilerError::SemanticError("Ternary condition must be a integer"));
        }

        let true_t = self.true_expr.verify(context)?;
        let false_t = self.false_expr.verify(context)?;

        if true_t != false_t {
            return Err(CompilerError::SemanticError("Ternary arms must have same type"));
        }

        Ok(true_t)
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub first: Expression,
    pub second: Expression,
}

impl BinaryExpr {
    pub fn verify(&self, context: &mut Context) -> Result<Type, CompilerError> {
        let first_t = self.first.verify(context)?;
        let second_t = self.second.verify(context)?;

        if first_t != second_t {
            return Err(CompilerError::SemanticError("Binary operator types must match"));
        }

        Ok(first_t)
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub expr: Expression
}

impl UnaryExpr {
    pub fn verify(&self, context: &mut Context) -> Result<Type, CompilerError> {
        self.expr.verify(context)
    }
}


#[derive(Debug)]
pub struct CastExpr {
    pub cast_to: Type,
    pub expr: Expression,
}

impl CastExpr {
    pub fn verify(&self, context: &mut Context) -> Result<Type, CompilerError> {
        // TODO: Verify that casting to given type is actually allowed
        self.expr.verify(context)?;
        Ok(self.cast_to.clone())
    }
}

#[derive(Debug)]
pub struct FuncCallExpr {
    pub func: Expression,
    pub args: Box<[Expression]>,
}

impl FuncCallExpr {
    pub fn verify(&self, context: &mut Context) -> Result<Type, CompilerError> {

        let f_type = self.func.verify(context)?;

        if let Type::Function(f) = f_type {
            for ((_, t_expected), arg) in f.args.iter().zip(&self.args) {
                let t_actual = arg.verify(context)?;

                if t_actual != *t_expected {
                    return Err(CompilerError::SemanticError("Incorrect function args"));
                }
            }

            Ok(f.return_type)
        } else {
            Err(CompilerError::SemanticError("Cannot call non-function"))
        }
    }
}

fn verify_bitshift(expr: &BinaryExpr, context: &mut Context) -> 
    Result<Type, CompilerError> {

    let val_t = expr.first.verify(context)?;
    let amt_t = expr.second.verify(context)?;

    if !amt_t.is_integer() {
        return Err(CompilerError::SemanticError("Cannot bitshift by a non-integer"));
    }

    Ok(val_t)
}

impl Expression {
    pub fn is_lvalue(&self) -> bool {
        match self {
            Self::Identifier(_) | Self::Dereference(_) | Self::ArrayIndex(_)
                => true,
            _ => false,
        }
    }

    /// Verifies the expression and returns its type
    pub fn verify(&self, context: &mut Context) -> Result<Type, CompilerError> {
        // TODO: Implicit casts
        match self {
            Self::Assignment(x) => {
                if !x.first.is_lvalue() {
                    return Err(CompilerError::SemanticError("Must assign to lvalue"));
                }

                x.verify(context)
            },

            Self::Ternary(x) => x.verify(context),

            Self::LogicalOr(x) => x.verify(context),
            Self::LogicalAnd(x) => x.verify(context),
            Self::BitwiseOr(x) => x.verify(context),
            Self::BitwiseXor(x) => x.verify(context),
            Self::BitwiseAnd(x) => x.verify(context),

            Self::Equality(x) => x.verify(context),
            Self::Inequality(x) => x.verify(context),

            Self::LTCompare(x) => x.verify(context),
            Self::GTCompare(x) => x.verify(context),
            Self::LECompare(x) => x.verify(context),
            Self::GECompare(x) => x.verify(context),

            Self::ShiftLeft(x) => verify_bitshift(x, context),
            Self::ShiftRight(x) => verify_bitshift(x, context),

            Self::Add(x) => x.verify(context),
            Self::Subtract(x) => x.verify(context),

            Self::Multiply(x) => x.verify(context),
            Self::Divide(x) => x.verify(context),
            Self::Modulo(x) => x.verify(context),

            Self::Cast(x) => x.verify(context),

            Self::PreInc(x) => x.verify(context),
            Self::PreDec(x) => x.verify(context),
            Self::PostInc(x) => x.verify(context),
            Self::PostDec(x) => x.verify(context),

            Self::AddressOf(x) => {
                if !x.expr.is_lvalue() {
                    return Err(CompilerError::SemanticError("Can't do & on a non-lvalue"));
                }

                Ok(Type::Pointer(Box::new(x.verify(context)?)))
            },

            Self::Dereference(x) => {
                let t = x.expr.verify(context)?;

                if let Type::Pointer(inner_t) = t {
                    Ok(*inner_t)
                } else {
                    Err(CompilerError::SemanticError("Cant dereference a non-pointer"))
                }
            },

            Self::Negate(x) => {
                let t = x.expr.verify(context)?;

                if !t.is_integer() {
                    return Err(CompilerError::SemanticError("Can't negate a non-integer"));
                }

                Ok(t)
            },

            Self::BitwiseNot(x) => x.verify(context),
            Self::LogicalNot(x) => x.verify(context),

            Self::SizeOf(x) => {
                x.verify(context)?;
                Ok(Type::Uint64)
            },

            Self::ArrayIndex(x) => {
                let array_t = x.first.verify(context)?;
                if let Type::Pointer(inner) = array_t {
                    let index_t = x.second.verify(context)?;

                    if !index_t.is_integer() {
                        return Err(CompilerError::SemanticError("Array index must be an integer"));
                    }

                    return Ok(*inner);
                };

                Err(CompilerError::SemanticError("Cannot index into non-array/pointer"))
            }

            Self::FuncCall(x) => x.verify(context),

            Self::Identifier(x) => match context.get_type(x) {
                Some(t) => Ok(t.clone()),
                None => Err(CompilerError::SemanticError("Undefined symbol")),
            }

            // TODO: Coerce literals into their actual type... somehow
            // For now we'll just make them all int32s and deal with it laterrr
            Self::IntLiteral(_) => Ok(Type::Int32),

            Self::CharLiteral(_) => Ok(Type::Uint8),
            Self::StringLiteral(_) => Ok(Type::Pointer(Box::new(Type::Uint8))),
        }
    }
}
