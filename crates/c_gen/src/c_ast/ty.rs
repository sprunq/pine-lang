use core::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct CTypedParam {
    pub name: String,
    pub ty: CType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CType {
    Void,

    U8,
    U16,
    U32,
    U64,
    USize,

    I8,
    I16,
    I32,
    I64,
    ISize,

    F32,
    F64,

    String,
    Struct(String),
    Pointer(Box<CType>),
    Reference(Box<CType>),
}

impl CType {
    pub fn points_to_struct(&self) -> bool {
        match self {
            CType::Pointer(ty) => matches!(&**ty, CType::Struct(_)),
            _ => false,
        }
    }
}

impl Display for CType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CType::Void => write!(f, "void"),
            CType::U8 => write!(f, "uint8_t"),
            CType::U16 => write!(f, "uint16_t"),
            CType::U32 => write!(f, "uint32_t"),
            CType::U64 => write!(f, "uint64_t"),
            CType::USize => write!(f, "size_t"),
            CType::I8 => write!(f, "int8_t"),
            CType::I16 => write!(f, "int16_t"),
            CType::I32 => write!(f, "int32_t"),
            CType::I64 => write!(f, "int64_t"),
            CType::ISize => write!(f, "ssize_t"),
            CType::F32 => write!(f, "float"),
            CType::F64 => write!(f, "double"),
            CType::String => write!(f, "char*"),
            CType::Struct(name) => write!(f, "{}", name),
            CType::Pointer(ty) => write!(f, "{}*", ty),
            CType::Reference(ty) => write!(f, "{}&", ty),
        }
    }
}
