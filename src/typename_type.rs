use crate::types::Type;

enum TypenameFlags {
    TNF_Plain = 1 << 0,
    TNF_Complete = 1 << 1,
}

struct TypenameType<'a> {
    storage_type: &'a Type,
    super_type: &'a Type,
    _name: &'a str,
    flags: u32
}