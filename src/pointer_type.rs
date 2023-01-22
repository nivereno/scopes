use crate::{types::Type, symbol::{Symbol, KnownSymbol}};

pub struct PointerType<'a> {
    this: &'a Type,
    element_type: &'a Type,
    flags: u64,
    storage_class: Symbol
}

impl <'a>PointerType<'a> {
    pub fn is_readable(&self) -> bool {
        return pointer_flags_is_readable(self.flags)
    }
    pub fn is_writable(&self) -> bool {
        return pointer_flags_is_writable(self.flags)
    }
}

enum PointerTypeFlags {
    PTF_NonWritable = 1 << 1,
    PTF_NonReadable = 1 << 2
}

fn pointer_flags_is_readable(flags: u64) -> bool {
    return (flags & PointerTypeFlags::PTF_NonReadable as u64) == 0
}
fn pointer_flags_is_writable(flags: u64) -> bool {
    return (flags & PointerTypeFlags::PTF_NonWritable as u64) == 0
}
fn pointer_storage_classes_compatible(need: Symbol, got: Symbol) -> bool {
    if need.0 == KnownSymbol::SYM_Unnamed as u64 {
        return true;
    }
    return need == got
}

pub fn native_opaque_pointer_type(element_type: &Type) -> &'static Type {
    todo!()
}
pub fn native_ro_pointer_type(element_type: &Type) -> Type {
    todo!()
}