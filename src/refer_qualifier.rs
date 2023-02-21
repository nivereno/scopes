use crate::{qualify_type::{Qualifier, QualifierKind}, symbol::Symbol, types::Type, pointer_type::pointer_type};


struct ReferQualifier {
    this: Qualifier,
    //enum { Kind = QK_Refer };
    flags: u64,
    storage_class: Symbol
}

impl ReferQualifier {
    pub fn new(_flags: u64, storage_class: Symbol) -> ReferQualifier {
        return ReferQualifier{ this: Qualifier::new(QualifierKind::QK_Refer), flags: _flags, storage_class: storage_class }
    }
    pub fn get_pointer_type(&self, ET: &Type) -> &Type {
        return pointer_type(ET, self.flags, self.storage_class.clone())
    }
}