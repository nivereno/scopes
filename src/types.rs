use std::{collections::HashMap, cell::RefCell};


use crate::{symbol::{Symbol, KnownSymbol}, valueref::ValueRef, typename_type::{TypenameType, incomplete_typename_type, opaque_typename_type, plain_typename_type}, pointer_type::{native_opaque_pointer_type, native_ro_pointer_type}};
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
    symbols: RefCell<HashMap<Symbol, TypeEntry>>
}
impl Default for Type {
    fn default() -> Self {
        //return Type { kind: (), symbols: HashMap::new() }
        todo!()
    }
}

struct B_Types<'a> {
    /* types */
    TYPE_Nothing: TypenameType<'a>,
    TYPE_NoReturn: TypenameType<'a>,
    TYPE_Variadic: TypenameType<'a>,
    //TYPE_Symbol: TypenameType<'a>,
    //TYPE_Builtin: TypenameType<'a>,
    //TYPE_ValueRef: TypenameType<'a>,

        // Just use Rust ints?
        /*TYPE_Bool: Type,
        TYPE_I8: Type,
        TYPE_I16: Type,
        TYPE_I32: Type,
        TYPE_I64: Type,
        TYPE_U8: Type,
        TYPE_U16: Type,
        TYPE_U32: Type,
        TYPE_U64: Type,
        TYPE_F16: Type,
        TYPE_F32: Type,
        TYPE_F64: Type,
        TYPE_F80: Type,
        TYPE_F128: Type,
        TYPE_Char: Type, */

    TYPE_Anchor: TypenameType<'a>,
    //TYPE_String: Type,
    //TYPE_Scope: Type,
    TYPE_SourceFile: TypenameType<'a>,
    //TYPE_ASTMacro: Type,
    //TYPE_CompileStage: Type,
    //TYPE_USize: Type,
    //TYPE_Sampler: Type,
    /* supertypes */
    TYPE_Immutable: TypenameType<'a>,
    TYPE_Aggregate: TypenameType<'a>,
    TYPE_OpaquePointer: TypenameType<'a>,
    TYPE_Pointer: TypenameType<'a>,
    _TypePtr: Type,
    TYPE__Value: TypenameType<'a>,
    TYPE_Closure: TypenameType<'a>,
    TYPE_Scope: TypenameType<'a>,
    TYPE_List: TypenameType<'a>,
    TYPE_Error: TypenameType<'a>,
    TYPE_Union: TypenameType<'a>,
    TYPE_Qualify: TypenameType<'a>,
    TYPE_Typename: TypenameType<'a>,
    TYPE_Arguments: TypenameType<'a>,
    TYPE_Raises: TypenameType<'a>,
    TYPE_Function: TypenameType<'a>,
    TYPE_Constant: TypenameType<'a>,
    TYPE_Image: TypenameType<'a>,
    TYPE_SampledImage: TypenameType<'a>,
    TYPE_CStruct: TypenameType<'a>,
    TYPE_CUnion: TypenameType<'a>,
    With_Supertypes: Option<B_Types_With_Supertypes<'a>>
}
impl <'a>Default for B_Types<'a> {
    fn default() -> Self {
        return B_Types { 
            TYPE_Nothing: incomplete_typename_type("nothing", None), 
            TYPE_NoReturn: opaque_typename_type("noreturn", None),
            TYPE_Variadic: opaque_typename_type("...", None), 
            // TYPE_Symbol: (), 
            // TYPE_Builtin: (),
            // TYPE_ValueRef: (),
            // TYPE_Char: (), 
            // TYPE_List: (), 
            // TYPE_String: (), 
            // TYPE_Scope: (), 
            // TYPE_SourceFile: (), 
            // TYPE_Error: (), 
            // TYPE_Closure: (), 
            // TYPE_ASTMacro: (), 
            // TYPE_CompileStage: (), 
            // TYPE_USize: (), 
            TYPE_Anchor: plain_typename_type("Anchor", None, native_opaque_pointer_type(&opaque_typename_type("_Anchor", None).this)).unwrap(),
            //TYPE_Sampler: todo!(), //sampler_type(), 
            TYPE_Immutable: opaque_typename_type("immutabe", None), 
            TYPE_Aggregate: opaque_typename_type("aggregate", None), 
            TYPE_OpaquePointer: opaque_typename_type("opaquepointer", None), 
            TYPE_Pointer: opaque_typename_type("pointer", None),
            _TypePtr: native_ro_pointer_type(&opaque_typename_type("_type", None).this),
            TYPE__Value: plain_typename_type("_Value", None, native_opaque_pointer_type(&opaque_typename_type("__Value", None).this)).unwrap(),
            TYPE_Closure: plain_typename_type("Closure", None, native_opaque_pointer_type(&opaque_typename_type("_Closure", None).this)).unwrap(),
            TYPE_Scope: plain_typename_type("Scope", None, native_opaque_pointer_type(&opaque_typename_type("_Scope", None).this)).unwrap(),
            TYPE_List: plain_typename_type("List", None, native_opaque_pointer_type(&opaque_typename_type("_List", None).this)).unwrap(),
            TYPE_Error: plain_typename_type("Error", None, native_opaque_pointer_type(&opaque_typename_type("_Error", None).this)).unwrap(),
            TYPE_Union: opaque_typename_type("union", None), 
            TYPE_Qualify: opaque_typename_type("Qualify", None), 
            TYPE_Typename: opaque_typename_type("typename", None), 
            TYPE_Arguments: opaque_typename_type("Arguments", None), 
            TYPE_Raises: opaque_typename_type("Raises", None), 
            TYPE_Function: opaque_typename_type("function", None), 
            TYPE_Constant: opaque_typename_type("Constant", None), 
            TYPE_Image: opaque_typename_type("Image", None), 
            TYPE_SampledImage: opaque_typename_type("SampledImage", None), 
            TYPE_CStruct: opaque_typename_type("CStruct", None), 
            TYPE_SourceFile: plain_typename_type("SourceFile", None, native_opaque_pointer_type(&opaque_typename_type("_SourceFile", None).this)).unwrap(),
            TYPE_CUnion: opaque_typename_type("CUnion", None),
            With_Supertypes: None,
        }
    }
}
struct B_Types_With_Supertypes<'a> {
    TYPE_Integer: TypenameType<'a>,
    TYPE_Real: TypenameType<'a>,
    TYPE_Vector: TypenameType<'a>,
    TYPE_Matrix: TypenameType<'a>,
    TYPE_Array: TypenameType<'a>,
    TYPE_ZArray: Option<TypenameType<'a>>,
    TYPE_Tuple: TypenameType<'a>,
    TYPE_Type: TypenameType<'a>,
    TYPE_Unknown: TypenameType<'a>,
    TYPE_String: TypenameType<'a>,
    TYPE_CEnum: TypenameType<'a>
}
impl <'a>B_Types<'a> {
    pub fn new(incomplete: &'a mut B_Types) -> &'a Self {
        let with_supertypes = B_Types_With_Supertypes {
            TYPE_Integer: opaque_typename_type("integer", Some(&incomplete.TYPE_Immutable.this)),
            TYPE_Real: opaque_typename_type("real", Some(&incomplete.TYPE_Immutable.this)),
            TYPE_Vector: opaque_typename_type("vector", Some(&incomplete.TYPE_Immutable.this)),
            TYPE_Matrix: opaque_typename_type("matrix", Some(&incomplete.TYPE_Immutable.this)),
            TYPE_Array: opaque_typename_type("array", Some(&incomplete.TYPE_Aggregate.this)),
            TYPE_ZArray: None,
            TYPE_Tuple: opaque_typename_type("tuple", Some(&incomplete.TYPE_Aggregate.this)),
            TYPE_Type: plain_typename_type("type", None, &incomplete._TypePtr).unwrap(),
            TYPE_Unknown: plain_typename_type("Unknown", None, &incomplete._TypePtr).unwrap(),
            TYPE_String: plain_typename_type("string", Some(&incomplete.TYPE_OpaquePointer.this), native_opaque_pointer_type(&opaque_typename_type("_string", None).this)).unwrap(),
            
            TYPE_CEnum: opaque_typename_type("Cenum", Some(&incomplete.TYPE_Immutable.this)),
        };
        incomplete.With_Supertypes = Some(with_supertypes);
        incomplete.With_Supertypes.as_mut().unwrap().TYPE_ZArray = Some(opaque_typename_type("zarray", Some(&incomplete.With_Supertypes.as_ref().unwrap().TYPE_Array.this)));
        return incomplete
    }
}


impl Type {
    fn kind(&self) -> TypeKind {
        return self.kind.clone()
    }
    pub fn new(kind: TypeKind) {
        
    }

    fn bind_with_doc(&mut self, name: Symbol, entry: &TypeEntry) {
        self.symbols.borrow_mut().insert(name, entry.clone());
    }
    fn bind(&mut self, name: Symbol, value: &ValueRef) {
        let entry: TypeEntry = TypeEntry{expr: value.clone(), doc: None};
        self.bind_with_doc(name, &entry);
    }

    fn unbind(&mut self, name: Symbol) {
        self.symbols.borrow_mut().remove(&name);
    }

    fn lookup_entry(&self, name: &Symbol, dest: &mut TypeEntry, types: &B_Types) -> bool {
        let mut T: Option<&Type> = Some(self);
        while let Some(Type) = T {
            if let Some(entry) = self.symbols.borrow_mut().get(name) {
                *dest = entry.clone();
                return true;
            }
            if *Type == types.TYPE_Typename.this {
                break;
            }
            T = superof(Type, types);
        }
        return false;
    }
    fn lookup_ref(&self, name: &Symbol, dest: &mut ValueRef, types: &B_Types) -> bool {
        let mut entry = TypeEntry{expr: ValueRef {value: crate::valueref::Value::usize(0), anchor: crate::anchor::Anchor::from(Symbol(0), 0, 0, 0)}, doc: None};
        if self.lookup_entry(name, &mut entry, types) {
            *dest = entry.expr;
            return true
        }
        return false
    }
    fn lookup_local_entry(&self, name: &Symbol, dest: &mut TypeEntry) -> bool {
        if let Some(entry) = self.symbols.borrow_mut().get(name) {
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
    fn lookup_call_handler(&self, dest: &mut ValueRef, types: &B_Types) -> bool {
        return self.lookup_ref(&Symbol(KnownSymbol::SYM_CallHandler as u64), dest, types);
    }
    fn lookup_return_handler(&self, dest: &mut ValueRef, types: &B_Types) -> bool {
        return self.lookup_ref(&Symbol(KnownSymbol::SYM_CallHandler as u64), dest, types);
    }
    fn lookup_quote_handler(&self, dest: &mut ValueRef, types: &B_Types) -> bool {
        return self.lookup_ref(&Symbol(KnownSymbol::SYM_CallHandler as u64), dest, types);
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
fn superof<'a>(T: &'a Type, types: &'a B_Types<'a>) -> Option<&'a Type> {
    match T.kind() {
        TypeKind::TK_Qualify => {return Some(&types.TYPE_Qualify.this)},
        TypeKind::TK_Arguments => {return Some(&types.TYPE_Arguments.this)},
        TypeKind::TK_Integer => {return Some(&types.With_Supertypes.as_ref().unwrap().TYPE_Integer.this)},
        TypeKind::TK_Real => {return Some(&types.With_Supertypes.as_ref().unwrap().TYPE_Real.this)},
        TypeKind::TK_Pointer => {return Some(&types.TYPE_Pointer.this)},
        TypeKind::TK_Array => {todo!()},//{return (cast<ArrayType>(T)->is_zterm())?TYPE_ZArray:TYPE_Array},
        TypeKind::TK_Vector => {return Some(&types.With_Supertypes.as_ref().unwrap().TYPE_Vector.this)},
        TypeKind::TK_Matrix => {return Some(&types.With_Supertypes.as_ref().unwrap().TYPE_Matrix.this)},
        TypeKind::TK_Tuple => {return Some(&types.With_Supertypes.as_ref().unwrap().TYPE_Tuple.this)},
        TypeKind::TK_Typename => {todo!()},//{return cast<TypenameType>(T)->super()},
        TypeKind::TK_Function => {return Some(&types.TYPE_Function.this)},
        TypeKind::TK_Image => {return Some(&types.TYPE_Image.this)},
        TypeKind::TK_SampledImage => {return Some(&types.TYPE_SampledImage.this)},
        TypeKind::TK_Sampler => {return Some(&types.TYPE_Immutable.this)},
        _ => {},
    }
    //return Err(anyhow!("unhandled type kind; corrupt pointer?"));
    return None
}
fn is_returning(T: &Type, types: &B_Types) -> bool {
    return *T != types.TYPE_NoReturn.this
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