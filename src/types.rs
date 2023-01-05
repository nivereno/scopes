use std::collections::HashMap;


use crate::{symbol::{Symbol, KnownSymbol}, valueref::ValueRef};
extern crate derive_more;
use anyhow::anyhow;
use derive_more::{Display};
#[derive(PartialEq, Clone, Display)]
pub enum TypeKind {
    /* abstract types */
    TK_Qualify,
    TK_Arguments,
    TK_Typename,
    /* machine types */
    TK_Integer,
    TK_Real,
    TK_Pointer,
    TK_Array,
    TK_Vector,
    TK_Matrix,
    TK_Tuple,
    TK_Function,
    /* additional GPU machine types */ 
    TK_Sampler,
    TK_Image,
    TK_SampledImage,
}
#[derive(PartialEq, Clone)]
struct TypeEntry {
    expr: ValueRef,
    doc: Option<String>
}
#[derive(PartialEq, Clone)]
pub struct Type {
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

    fn lookup_entry(&self, name: &Symbol, dest: &mut TypeEntry) -> bool {
        let mut T: Option<&Type> = Some(self);
        while let Some(Type) = T {
            if let Some(entry) = self.symbols.get(name) {
                *dest = entry.clone();
                return true;
            }
            //if Type == TYPE_Typename {
            //    break;
            //}
            T = superof(Type);
        }
        return false;
    }
    fn lookup_ref(&self, name: &Symbol, dest: &mut ValueRef) -> bool {
        let mut entry = TypeEntry{expr: ValueRef {value: crate::valueref::Value::usize(0), anchor: crate::anchor::Anchor::from(Symbol(0), 0, 0, 0)}, doc: None};
        if self.lookup_entry(name, &mut entry) {
            *dest = entry.expr;
            return true
        }
        return false
    }
    fn lookup_local_entry(&self, name: &Symbol, dest: &mut TypeEntry) -> bool {
        if let Some(entry) = self.symbols.get(name) {
            *dest = entry.clone();
            return true;
        }
        return false;
    }
    fn lookup_local_ref(&self, name: &Symbol, dest: &mut ValueRef) -> bool {
        let mut entry = TypeEntry{expr: ValueRef {value: crate::valueref::Value::usize(0), anchor: crate::anchor::Anchor::from(Symbol(0), 0, 0, 0)}, doc: None};
        if self.lookup_local_entry(name, &mut entry) {
            *dest = entry.expr;
            return true;
        }
        return false;
    }
    fn lookup_call_handler(&self, dest: &mut ValueRef) -> bool {
        return self.lookup_ref(&Symbol(KnownSymbol::SYM_CallHandler as u64), dest);
    }
    fn lookup_return_handler(&self, dest: &mut ValueRef) -> bool {
        return self.lookup_ref(&Symbol(KnownSymbol::SYM_CallHandler as u64), dest);
    }
    fn lookup_quote_handler(&self, dest: &mut ValueRef) -> bool {
        return self.lookup_ref(&Symbol(KnownSymbol::SYM_CallHandler as u64), dest);
    }
}

fn is_opaque(T: &Type) -> bool {
    match T.kind() {
        TypeKind::TK_Qualify => {todo!()},//return is_opaque(cast<QualifyType>(T)->type)},
        TypeKind::TK_Typename => {
            /*const TypenameType *tt = cast<TypenameType>(T);
            if (tt->is_opaque()) {
                return true;
            } else {
                // does this make sense?
                return is_opaque(tt->storage());
            }*/
            todo!()
        },
            //case TK_Image: // can be loaded
            //case TK_SampledImage: // can be loaded
            TypeKind::TK_Function | TypeKind::TK_Arguments => {return true},
            _ => {return false}
        }
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
    return size_of(qualified_storage_type(T)?)
}
fn align_of(T: &Type) -> Result<usize, anyhow::Error> {
    todo!()
}
fn qualified_align_of(T: &Type) -> Result<usize, anyhow::Error> {
    return align_of(qualified_storage_type(T)?)
}
fn superof(T: &Type) -> Option<&Type> {
    todo!()
    /*match T.kind() {
        TypeKind::TK_Qualify => {return TYPE_Qualify},
        TypeKind::TK_Arguments => return TYPE_Arguments},
        TypeKind::TK_Integer => {return TYPE_Integer},
        TypeKind::TK_Real => {return TYPE_Real},
        TypeKind::TK_Pointer => {return TYPE_Pointer},
        TypeKind::TK_Array => {return (cast<ArrayType>(T)->is_zterm())?TYPE_ZArray:TYPE_Array},
        TypeKind::TK_Vector => {return TYPE_Vector},
        TypeKind::TK_Matrix => {return TYPE_Matrix},
        TypeKind::TK_Tuple => {return TYPE_Tuple},
        TypeKind::TK_Typename => {return cast<TypenameType>(T)->super()},
        TypeKind::TK_Function => {return TYPE_Function},
        TypeKind::TK_Image => {return TYPE_Image},
        TypeKind::TK_SampledImage => {return TYPE_SampledImage},
        TypeKind::TK_Sampler => {return TYPE_Immutable},
    }
    return Err(anyhow!("unhandled type kind; corrupt pointer?"))*/
}
fn is_returning(T: &Type) -> bool {
    //return T != TYPE_NoReturn
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
    //use TypeKind
    loop {
        match T.kind() {
            TypeKind::TK_Qualify => {todo!()},
            TypeKind::TK_Pointer => {todo!()},
            TypeKind::TK_Integer | TypeKind::TK_Real | TypeKind::TK_Image | TypeKind::TK_SampledImage | TypeKind::TK_Sampler | TypeKind::TK_Function => {return true},
            TypeKind::TK_Array | TypeKind::TK_Vector | TypeKind::TK_Matrix => {todo!()},
            TypeKind::TK_Tuple => {todo!()},
            TypeKind::TK_Arguments => {todo!()},
            TypeKind::TK_Typename => {todo!()}
        }
    }
    return false
} 

fn storage_type(T: &Type) -> Result<&Type, anyhow::Error> {
    /*T = strip_qualifiers(T);
    match T.kind() {
        TK_Typename => {
            let tt = T;
            todo!()
            //const TypenameType *tt = cast<TypenameType>(T);
            if !tt.is_complete() {
                return anyhow!("TypenameIncomplete {T}")
            }
            if tt.is_opaque() {
                return anyhow!("OpaqueType {T}")
            }
            return tt.storage();
            }
        _ => {return Ok(T)}
    }*/
    todo!()
}

fn qualified_storage_type(T: &Type) -> Result<&Type, anyhow::Error> {
    /*let rq = try_qualifier<ReferQualifier>(T);
    if rq {
        T = strip_qualifiers(T);
        return pointer_type(T, rq.flags, rq.storage_class);
    } else {
        return storage_type(T);
    }*/
    todo!()
}
//------------------------------------------------------------------------------
// TYPE CHECK PREDICATES
//------------------------------------------------------------------------------

fn verify(need: &Type, have: &Type) -> Result<(), anyhow::Error> {
    //if strip_lifetime(need) != strip_lifetime(have) {
        let need = need.kind();
        let have = have.kind();
        return Err(anyhow!("ParameterTypeMismatch {need}, {have}"))
    //}
    //return Ok(())
}

fn verify_integer(T: &Type) -> Result<(), anyhow::Error> {
    if T.kind() != TypeKind::TK_Integer {
        let T = T.kind();
        return Err(anyhow!("ParameterTypeMismatch TYPE_Integer, {T}"))
    }
    return Ok(())
}

fn verify_real(T: &Type) -> Result<(), anyhow::Error> {
    if T.kind() != TypeKind::TK_Real {
        let T = T.kind();
        return Err(anyhow!("ParameterTypeMismatch TYPE_Real {T}"))
    }
    return Ok(())
}

fn verify_range(idx: usize, count: usize) -> Result<(), anyhow::Error> {
    if idx >= count {
        return Err(anyhow!("IndexOutOfRange {idx}, {count}"))
    }
    return Ok(())
}

//------------------------------------------------------------------------------