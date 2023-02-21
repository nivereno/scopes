use crate::{qualify_type::Qualifier, symbol::Symbol, types::Type, pointer_type::pointer_type};


struct ReferQualifier {
    this: Qualifier,
    //enum { Kind = QK_Refer };
    flags: u64,
    storage_class: Symbol
}

impl ReferQualifier {
    pub fn new() {

    }
    pub fn get_pointer_type(&self, ET: &Type) -> &Type {
        return pointer_type(ET, self.flags, self.storage_class.clone())
    }
}