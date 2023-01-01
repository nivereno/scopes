use std::collections::HashMap;

use crate::{symbol::{Symbol, KnownSymbol}, valueref::ValueRef};
#[derive(PartialEq, Clone)]
enum TypeKind {

}
#[derive(PartialEq, Clone)]
struct TypeEntry {
    expr: ValueRef,
    doc: Option<String>
}
#[derive(PartialEq, Clone)]
struct Type {
    kind: TypeKind,
    symbols: HashMap<Symbol, TypeEntry>
}

impl Type {
    fn kind(&self) -> TypeKind {
        return self.kind.clone()
    }
    pub fn new(kind: TypeKind) {
        
    }

    fn bind_with_doc(&mut self, name: Symbol, entry: &TypeEntry) {
        self.symbols.insert(name, entry.clone());
    }
    fn bind(&mut self, name: Symbol, value: &ValueRef) {
        let entry: TypeEntry = TypeEntry{expr: value.clone(), doc: None};
        self.bind_with_doc(name, &entry);
    }

    fn unbind(&mut self, name: Symbol) {
        self.symbols.remove(&name);
    }

    fn lookup_entry(&self, name: Symbol, dest: &TypeEntry) -> bool {
        todo!()
    }
    fn lookup_ref(&self, name: Symbol, dest: &ValueRef) -> bool {
        todo!()
    }
    fn lookup_local_entry(name: Symbol, dest: &TypeEntry) -> bool {
        todo!()
    }
    fn lookup_local_ref(name: Symbol, dest: &ValueRef) -> bool {
        todo!()
    }
    fn lookup_call_handler(&self, dest: &ValueRef) -> bool {
        return self.lookup_ref(Symbol(KnownSymbol::SYM_CallHandler as u64), dest);
    }
    fn lookup_return_handler(&self, dest: &ValueRef) -> bool {
        return self.lookup_ref(Symbol(KnownSymbol::SYM_CallHandler as u64), dest);
    }
    fn lookup_quote_handler(&self, dest: &ValueRef) -> bool {
        return self.lookup_ref(Symbol(KnownSymbol::SYM_CallHandler as u64), dest);
    }
}

fn is_opaque(T: &Type) -> bool {
    todo!()
}
fn storage_kind(T: &Type) -> TypeKind {
    if is_opaque(T) {
        return T.kind()
    }
    return storage_type(T).unwrap().kind();
}
fn size_of(T: &Type) -> Result<usize, anyhow::Error> {
    todo!()
}
fn bitsize_of(T: &Type) -> Result<usize, anyhow::Error> {
    todo!()
}
fn qualified_size_of(T: &Type) -> Result<usize, anyhow::Error> {
    todo!()
}
fn align_of(T: &Type) -> Result<usize, anyhow::Error> {
    todo!()
}
fn qualified_align_of(T: &Type) -> Result<usize, anyhow::Error> {
    todo!()
}
fn superof(T: &Type) -> &Type {
    todo!()
}
fn is_returning(T: &Type) -> bool {
    todo!()
}
fn is_returning_value(T: &Type) -> bool {
    todo!()
    //return is_returning(T) && T != empty_arguments_type()
}
fn types_compatible(paramT: &Type, argT: &Type) -> bool {
    todo!()
}
fn all_plain(types: &Vec<Type>) -> bool {
    for t in types {
        if !is_plain(t) {
            return false
        }
    }
    return true
}
// can be copied implicitly, without needing a copy constructor
fn is_plain(T: &Type) -> bool {
    match T.kind() {

    }

    return false
} 

fn storage_type(T: &Type) -> Result<&Type, anyhow::Error> {
    todo!()
}