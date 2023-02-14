use std::{collections::HashMap, collections::HashSet, cell::RefCell};

use crate::{types::{Type, is_plain, B_Types, TypeKind}, symbol::{Symbol, SymbolMap, self}};
use anyhow::{anyhow};
use derive_more::{Display};

thread_local! {static used_names: RefCell<HashSet<Symbol>> = RefCell::new(HashSet::new())}
enum TypenameFlags {
    TNF_Plain = 1 << 0,
    TNF_Complete = 1 << 1,
}

#[derive(PartialEq, Clone)]
pub struct TypenameType<'a> {
    pub this: Type,
    storage_type: Option<&'a Type>,
    super_type: Option<&'a Type>,
    _name: &'a str,
    flags: u32
}

impl <'a>TypenameType<'a> {
    fn new(name: &str, _super_type: Option<&Type>) -> TypenameType<'a> {
        let this = Type{ kind: TypeKind::TK_Typename, symbols: RefCell::new(HashMap::new()) };
        //let newname = symbols.add_symbol(name);
        //return TypenameType { this: this, storage_type: None, super_type: Some(_super_type), _name: newname, flags: 0 }
        todo!()
    }
    pub fn name(&self) -> &str {
        return self._name
    }
    pub fn storage(&self) -> Option<&Type> {
        return self.storage_type
    }
    pub fn complete(&mut self) -> Result<(), anyhow::Error> {
        if self.is_complete() {
            return Err(anyhow!("TypenameComplete"))
        }
        self.flags = TypenameFlags::TNF_Complete as u32;
        return Ok(())
    }
    pub fn complete_set(&mut self, _type: &'a Type, mut _flags: u32) -> Result<(), anyhow::Error> {
        _flags |= TypenameFlags::TNF_Complete as u32;
        if self.is_complete() {
            anyhow!("StorageTypeExpected");
        }
        if false { //isa<TypenameType>(_type)
            todo!()
        }
        if (_flags & TypenameFlags::TNF_Plain as u32) != 0 && !is_plain(_type) {
            anyhow!("PlainStorageTypeExpected");
        }
        self.storage_type = Some(_type);
        self.flags = _flags;
        return Ok(());
    }
    fn is_opaque(&self) -> bool {
        return self.storage_type == None
    }
    fn is_complete(&self) -> bool {
        return (self.flags & TypenameFlags::TNF_Plain as u32) == TypenameFlags::TNF_Complete as u32
    }
    fn is_plain(&self) -> bool {
        return self.is_opaque() || ((self.flags & TypenameFlags::TNF_Plain as u32) == TypenameFlags::TNF_Plain as u32)
    }
    fn super_type(&'a self, types: &'a B_Types) -> Option<&Type> {
        if self.super_type == None {
            return Some(&types.TYPE_Typename.this);
        }
        return self.super_type
    }
}

// always generates a new type
pub fn incomplete_typename_type<'a>(name: &str, supertype: Option<&Type>) -> TypenameType<'a> {
    return TypenameType::new(name, supertype)
}

pub fn opaque_typename_type<'a>(name: &str, supertype: Option<&Type>) -> TypenameType<'a> {
    let mut TT = incomplete_typename_type(name, supertype);
    match TT.complete() {
        Ok(_) => {return TT},
        Err(TT) => {eprintln!("{TT}")}
    }
    return TT;
}

pub fn plain_typename_type<'a>(name: &str, supertype: Option<&Type>, storage_type: &'a Type) -> Result<TypenameType<'a>, anyhow::Error> {
    let mut TT = incomplete_typename_type(name, supertype);
    TT.complete_set(storage_type, TypenameFlags::TNF_Plain as u32)?;
    return Ok(TT)
}

pub fn unique_typename_type<'a>(name: &str, supertype: Option<&Type>, storage_type: &'a Type) -> Result<TypenameType<'a>, anyhow::Error> {
    let mut TT = incomplete_typename_type(name, supertype);
    TT.complete_set(storage_type, 0)?;
    return Ok(TT)
}