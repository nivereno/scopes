use std::{cell::RefCell, collections::HashSet};

use crate::{types::Type, symbol::Symbol};

thread_local!(static images: RefCell<HashSet<Box<ImageType<'static>>>> = RefCell::new(HashSet::new()));

pub struct ImageType<'a> {
    T: &'a Type,
    this: Type,
    dim: Symbol,
    depth: isize,
    arrayed: isize,
    multisampled: isize,
    sampled: isize,
    format: Symbol,
    access: Symbol
}
impl <'a>ImageType<'a> {
    pub fn new(_type: &Type, _dim: Symbol, _depth: isize, _arrayed: isize, _multisampled: isize, _sampled: isize, _format: Symbol, _acess: Symbol) -> ImageType {
        return ImageType { T: _type, this: Type::new(crate::types::TypeKind::TK_Image), dim: _dim, depth: _depth, arrayed: _arrayed, multisampled: _multisampled, sampled: _sampled, format: _format, access: _acess }
    }
}