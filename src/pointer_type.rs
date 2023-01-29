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

fn required_flags_for_storage_class(storage_class: Symbol) -> u64 {
    let SYM_Unnamed = KnownSymbol::SYM_Unnamed as u64;
    let SYM_SPIRV_StorageClassUniformConstant = KnownSymbol::SYM_SPIRV_StorageClassUniformConstant as u64;
    let SYM_SPIRV_StorageClassInput = KnownSymbol::SYM_SPIRV_StorageClassInput as u64;
    let SYM_SPIRV_StorageClassUniform = KnownSymbol::SYM_SPIRV_StorageClassUniform as u64;
    let SYM_SPIRV_StorageClassOutput = KnownSymbol::SYM_SPIRV_StorageClassOutput as u64;
    let SYM_SPIRV_StorageClassWorkgroup = KnownSymbol::SYM_SPIRV_StorageClassWorkgroup as u64;
    let SYM_SPIRV_StorageClassCrossWorkgroup = KnownSymbol::SYM_SPIRV_StorageClassCrossWorkgroup as u64;
    let SYM_SPIRV_StorageClassPrivate = KnownSymbol::SYM_SPIRV_StorageClassPrivate;
    let SYM_SPIRV_StorageClassFunction = KnownSymbol::SYM_SPIRV_StorageClassFunction as u64;
    let SYM_SPIRV_StorageClassGeneric = KnownSymbol::SYM_SPIRV_StorageClassGeneric as u64;
    let SYM_SPIRV_StorageClassPushConstant = KnownSymbol::SYM_SPIRV_StorageClassPushConstant as u64;
    let SYM_SPIRV_StorageClassAtomicCounter = KnownSymbol::SYM_SPIRV_StorageClassAtomicCounter as u64;
    let SYM_SPIRV_StorageClassImage = KnownSymbol::SYM_SPIRV_StorageClassImage as u64;
    let SYM_SPIRV_StorageClassStorageBuffer = KnownSymbol::SYM_SPIRV_StorageClassStorageBuffer as u64;

    match storage_class.value() {
        SYM_Unnamed => return 0,
        SYM_SPIRV_StorageClassUniformConstant => return PointerTypeFlags::PTF_NonWritable as u64,
        SYM_SPIRV_StorageClassInput => return PointerTypeFlags::PTF_NonWritable as u64,
        SYM_SPIRV_StorageClassUniform => return 0,
        SYM_SPIRV_StorageClassOutput => return PointerTypeFlags::PTF_NonReadable as u64,
        SYM_SPIRV_StorageClassWorkgroup => return 0,
        SYM_SPIRV_StorageClassCrossWorkgroup => return 0,
        SYM_SPIRV_StorageClassPrivate => return 0,
        SYM_SPIRV_StorageClassFunction => return 0,
        SYM_SPIRV_StorageClassGeneric => return 0,
        SYM_SPIRV_StorageClassPushConstant => PointerTypeFlags::PTF_NonWritable as u64,
        SYM_SPIRV_StorageClassAtomicCounter => return 0,
        SYM_SPIRV_StorageClassImage => return 0,
        SYM_SPIRV_StorageClassStorageBuffer => return 0,
        _ => return PointerTypeFlags::PTF_NonWritable as u64 | PointerTypeFlags::PTF_NonReadable as u64
    }
}
pub fn pointer_type(element_type: &Type, flags: u64, storage_class: Symbol) -> &'static Type {

    todo!()
}
pub fn native_opaque_pointer_type(element_type: &Type) -> &'static Type {
    return pointer_type(element_type, PointerTypeFlags::PTF_NonWritable as u64 | PointerTypeFlags::PTF_NonReadable as u64, Symbol(KnownSymbol::SYM_Unnamed as u64));
}
pub fn native_ro_pointer_type(element_type: &Type) -> &Type {
    return pointer_type(element_type, PointerTypeFlags::PTF_NonWritable as u64, Symbol(KnownSymbol::SYM_Unnamed as u64));
}
pub fn native_pointer_type(element_type: &Type) -> &Type {
    return pointer_type(element_type, 0, Symbol(KnownSymbol::SYM_Unnamed as u64));
}
pub fn local_ro_pointer_type(element_type: &Type) -> &Type {
    return pointer_type(element_type, PointerTypeFlags::PTF_NonWritable as u64, Symbol(KnownSymbol::SYM_SPIRV_StorageClassFunction as u64));
}
pub fn local_pointer_type(element_type: &Type) -> &Type {
    return pointer_type(element_type, 0, Symbol(KnownSymbol::SYM_SPIRV_StorageClassFunction as u64));
}
pub fn static_pointer_type(element_type: &Type) -> &Type {
    return pointer_type(element_type, 0, Symbol(KnownSymbol::SYM_SPIRV_StorageClassFunction as u64));
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
fn pointer_flags_compatible(need: u64, got: u64) -> bool {
    /*
        need     got-> | 0 | nowrite | noread | nowrite-noread |
        0              | Y |    N    |    N   |        N       |
        nowrite        | Y |    Y    |    N   |        N       |
        noread         | Y |    N    |    Y   |        N       |
        nowrite-noread | Y |    Y    |    Y   |        Y       |
    */
    if got == 0 {
        return true
    } 
    if need == (PointerTypeFlags::PTF_NonWritable as u64 | PointerTypeFlags::PTF_NonReadable as u64) {
        return true
    }
    return got == need;
}