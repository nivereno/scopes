enum TypeKind {

}

struct TypeEntry {
    expr: ValueRef,
    doc: &str
}

struct Type {
    kind: TypeKind,
    symbols: HashMap<Symbol, TypeEntry>
}

impl Type {
    fn kind() -> TypeKind {

    }
    pub fn new(kind: TypeKind) {

    }

    fn bind_with_doc(name: Symbol, entry: &TypeEntry) {

    }
    fn bind(name: Symbol, value: &ValueRef) {

    }

    fn unbind(name: Symbol) {
        
    }

    fn lookup(name: Symbol, dest: &TypeEntry) -> bool {

    }
    fn lookup(name: Symbol, dest: &ValueRef) -> bool {

    }
    fn lookup_local(name: Symbol, dest: &TypeEntry) -> bool {

    }
    fn lookup_local(name: Symbol, dest: &ValueRef) -> bool {

    }
    fn lookup_call_handler(dest: &ValueRef) -> bool {

    }
    fn lookup_return_handler(dest: &ValueRef) -> bool {

    }
    fn lookup_quote_handler(dest: &ValueRef) -> bool {

    }
}

fn is_opaque(T: &Type) -> bool {

}
fn storage_kind(T: &Type) -> TypeKind {

}
fn size_of(T: &Type) -> Result<usize, anyhow::Error> {

}
fn bitsize_of(T: &Type) -> Result<usize, anyhow::Error> {

}
fn qualified_size_of(T: &Type) -> Result<usize, anyhow::Error> {

}
fn align_of(T: &Type) -> Result<usize, anyhow::Error> {

}
fn qualified_align_of(T: &Type) -> Result<usize, anyhow::Error> {
    
}
fn superof(T: &Type) -> &Type {

}
fn is_returning(T: &Type) -> bool {
    
}
fn is_returning_value(T: &Type) -> bool {
    
}
fn types_compatible(paramT: &Type, argT: &Type) -> bool {
    
}
fn all_plain(types: &Type) -> bool {
    
}
// can be copied implicitly, without needing a copy constructor
fn is_plain(T: &Type) -> bool {

}