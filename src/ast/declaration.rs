use super::{Expression, Statement, Type};

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub type_of: Type,
    pub value: Option<DeclarationValue>,
}

#[derive(Debug)]
pub enum DeclarationValue {
    Variable(Expression),
    Function(Box<[Statement]>),
}
