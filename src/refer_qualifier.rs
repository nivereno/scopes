use std::{cell::RefCell, collections::HashSet};

use crate::{qualify_type::{Qualifier, QualifierKind, qualify}, symbol::{Symbol, KnownSymbol}, types::Type, pointer_type::{pointer_type, required_flags_for_storage_class, required_flags_for_element_type}, qualifier::{has_qualifier, try_qualifier}, all_types::All_types};

thread_local!(static refers: RefCell<HashSet<Box<ReferQualifier>>> = RefCell::new(HashSet::new()));

#[derive(Hash, PartialEq, Eq, Clone)]
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

pub fn refer_type(T: All_types, mut flags: u64, storage_class: Symbol) -> All_types {
    flags |= required_flags_for_storage_class(&storage_class);
    //flags |= required_flags_for_element_type(T);
    todo!();
    return refers.with(|set| {
        let key = Box::new(ReferQualifier::new(flags, storage_class));
        if let Some(result) = set.borrow().get(&key) {
            return qualify(T, vec!(&result.this))
        }
        let qualifiers = key.this.clone();
        set.borrow_mut().insert(key);
        return qualify(T, vec!(&qualifiers))
    });
}
pub fn refer_flags(T: All_types) -> u64 {
    if let Some(q) = try_qualifier(T) {
        //return q.flags
        todo!()
    }
    return 0
}
pub fn refer_storage_class(T: All_types) -> Symbol {
    if let Some(q) = try_qualifier(T) {
        //return q.storage_class
        todo!()
    }  
    return Symbol(KnownSymbol::SYM_Unnamed as u64)
}
pub fn is_reference(T: All_types) -> bool {
    return has_qualifier(T);
    todo!()
}


/* 
uint64_t refer_flags(const Type *T) {
    auto q = try_qualifier<ReferQualifier>(T);
    if (q) { return q->flags; }
    return 0;
}

Symbol refer_storage_class(const Type *T) {
    auto q = try_qualifier<ReferQualifier>(T);
    if (q) { return q->storage_class; }
    return SYM_Unnamed;
}

bool is_reference(const Type *T) {
    return has_qualifier<ReferQualifier>(T);
}*/