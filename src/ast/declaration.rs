use super::{Expression, Statement, Type};

pub struct Declaration {
    pub name: String,
    pub type_of: Type,
    pub value: Option<DeclarationValue>,
}

pub enum DeclarationValue {
    Variable(Expression),
    Function(Box<[Statement]>),
}
