use crate::ast::Type;

pub fn get_size(t: &Type) -> usize {
    match t {
        Type::Uint8  | Type::Int8  => 1,
        Type::Uint16 | Type::Int16 => 2,
        Type::Uint32 | Type::Int32 => 4,
        Type::Uint64 | Type::Int64 => 8,

        Type::Float | Type::Double => todo!(),

        Type::Pointer(_) => 8,

        Type::Void | Type::Function(_) => panic!("Shouldn't be sizing void or fn"),
    }
}
