use crate::types::Type;
use anyhow::anyhow;
use derive_more::{Display};
enum TypenameFlags {
    TNF_Plain = 1 << 0,
    TNF_Complete = 1 << 1,
}

struct TypenameType<'a> {
    storage_type: Option<&'a Type>,
    super_type: Option<&'a Type>,
    _name: &'a str,
    flags: u32
}

impl <'a>TypenameType<'a> {
    fn new() -> TypenameType<'a> {
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
    pub fn complete_set(&self, _type: &Type, _flags: u32) -> Result<(), anyhow::Error> {
        todo!()
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
    fn super_type(&self) -> Option<&Type> {
        if self.super_type == None {
            //return TYPE_Typename
            todo!()
        }
        return self.super_type
    }
}

// always generates a new type
fn incomplete_typename_type<'a>(name: &str, supertype: &Type) -> TypenameType<'a> {
    return TypenameType::new()//{_name: name, super_type: Some(supertype)};
}

fn opaque_typename_type<'a>(name: &str, supertype: &Type) -> TypenameType<'a> {
    let mut TT = incomplete_typename_type(name, supertype);
    match TT.complete() {
        Ok(_) => {return TT},
        Err(TT) => {eprintln!("{TT}")}
    }
    return TT;
}

fn plain_typename_type<'a>(name: &str, supertype: &Type, storage_type: &Type) -> Result<TypenameType<'a>, anyhow::Error> {
    let mut TT = incomplete_typename_type(name, supertype);
    TT.complete_set(storage_type, TypenameFlags::TNF_Plain as u32)?;
    return Ok(TT)
}

fn unique_typename_type<'a>(name: &str, supertype: &Type, storage_type: &Type) -> Result<TypenameType<'a>, anyhow::Error> {
    let mut TT = incomplete_typename_type(name, supertype);
    TT.complete_set(storage_type, 0)?;
    return Ok(TT)
}