#[derive(Debug, Clone)]
pub enum Type {
    Uint8, // unsigned char
    Uint16,// unsigned short
    Uint32,// unsigned int
    Uint64,// unsigned long
    Int8,  // signed char
    Int16, // signed short
    Int32, // signed int
    Int64, // signed long
    
    // Not currently supported cause idk how floating-point stuff works
    Float,
    Double,

    // Arrays are pointers
    Pointer(Box<Type>),

    Function(Box<FunctionType>),

    // TODO: Structs, enums
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub return_type: Option<Type>,
    pub args: Box<[(String, Type)]>,
}
