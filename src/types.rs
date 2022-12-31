use std::collections::HashMap;

use crate::{symbol::Symbol, valueref::ValueRef};
#[derive(PartialEq, Clone)]
enum TypeKind {

}
#[derive(PartialEq, Clone)]
struct TypeEntry {
    expr: ValueRef,
    doc: String
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
    fn bind(name: Symbol, value: &ValueRef) {

    }

    fn unbind(name: Symbol) {
        
    }

    fn lookup_entry(name: Symbol, dest: &TypeEntry) -> bool {
        todo!()
    }
    fn lookup_ref(name: Symbol, dest: &ValueRef) -> bool {
        todo!()
    }
    fn lookup_local_entry(name: Symbol, dest: &TypeEntry) -> bool {
        todo!()
    }
    fn lookup_local_ref(name: Symbol, dest: &ValueRef) -> bool {
        todo!()
    }
    fn lookup_call_handler(dest: &ValueRef) -> bool {
        todo!()
    }
    fn lookup_return_handler(dest: &ValueRef) -> bool {
        todo!()
    }
    fn lookup_quote_handler(dest: &ValueRef) -> bool {
        todo!()
    }
}

fn is_opaque(T: &Type) -> bool {
    todo!()
}
fn storage_kind(T: &Type) -> TypeKind {
    todo!()
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
}
fn types_compatible(paramT: &Type, argT: &Type) -> bool {
    todo!()
}
fn all_plain(types: &Type) -> bool {
    todo!()
}
// can be copied implicitly, without needing a copy constructor
fn is_plain(T: &Type) -> bool {
    todo!()
}    