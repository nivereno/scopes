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