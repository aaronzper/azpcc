#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Void,

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

impl Type {
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Uint8 | Self::Uint16 | Self::Uint32 | Self::Uint64 |
            Self::Int8  | Self::Int16  | Self::Int32  | Self::Int64 => true,

            _ => false
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            Self::Int8 | Self::Int16 | Self::Int32 | Self::Int64 => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct FunctionType {
    pub return_type: Type,
    pub args: Box<[(String, Type)]>,
}

impl PartialEq for FunctionType {
    fn eq(&self, other: &Self) -> bool {
        if self.return_type != other.return_type {
            return false;
        }

        // Doesnt matter if arg names match, only types
        for ((_, t1), (_, t2)) in self.args.iter().zip(&other.args) {
            if t1 != t2 {
                return false;
            }
        }

        true
    }
}
