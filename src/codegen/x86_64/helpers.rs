use crate::ast::Type;

use super::registers::RegisterSize;

pub fn get_bytes(t: &Type) -> usize {
    match t {
        Type::Void => 0,

        Type::Uint8  | Type::Int8  => 1,
        Type::Uint16 | Type::Int16 => 2,
        Type::Uint32 | Type::Int32 => 4,
        Type::Uint64 | Type::Int64 => 8,

        Type::Float | Type::Double => todo!(),

        Type::Pointer(_) => 8,

        Type::Function(_) => panic!("Shouldn't be sizing fn"),
    }
}

pub fn get_size(t: &Type) -> RegisterSize {
    match get_bytes(t) {
        0 => RegisterSize::Void,
        1 => RegisterSize::Byte,
        2 => RegisterSize::Word,
        4 => RegisterSize::DWord,
        8 => RegisterSize::QWord,
        _ => panic!("Invalid size"),
    }
}

pub fn get_global_asm(symbol: &str, type_of: &Type) -> String {
    let is_fn = match type_of {
        Type::Function(_) => true,
        _ => false,
    };

    if is_fn {
        symbol.to_string()
    } else {
        format!("{} [{}]", get_size(type_of), symbol)
    }
}
