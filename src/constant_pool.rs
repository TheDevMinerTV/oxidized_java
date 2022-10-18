pub const CLASS: u8 = 7;
pub const FIELDREF: u8 = 9;
pub const METHODREF: u8 = 10;
pub const INTERFACE_METHODREF: u8 = 11;
pub const STRING: u8 = 8;
pub const INTEGER: u8 = 3;
pub const FLOAT: u8 = 4;
pub const LONG: u8 = 5;
pub const DOUBLE: u8 = 6;
pub const NAME_AND_TYPE: u8 = 12;
pub const UTF8: u8 = 1;
pub const METHOD_HANDLE: u8 = 15;
pub const METHOD_TYPE: u8 = 16;
pub const INVOKE_DYNAMIC: u8 = 18;

pub enum Constant {
    Class(Class),
    Fieldref(Fieldref),
    Methodref(Methodref),
    InterfaceMethodref,
    String(String),
    Integer,
    Float,
    Long,
    Double,
    NameAndType(NameAndType),
    Utf8(Utf8),
    MethodHandle,
    MethodType,
    InvokeDynamic,
}

pub struct Methodref {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

pub struct Class {
    pub name_index: u16,
}

pub struct NameAndType {
    pub name_index: u16,
    pub descriptor_index: u16,
}

pub struct Utf8 {
    pub length: u16,
    pub bytes: Vec<u8>,
}

pub struct Fieldref {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

pub struct String {
    pub string_index: u16,
}
