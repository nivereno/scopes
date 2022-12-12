use crate::symbol::Symbol;
use crate::symbol::SymbolMap;


#[derive(PartialEq, Clone)]
pub struct Anchor {
    path: Symbol,
    lineno: usize,
    offset: usize,
    buffer: usize
}
//TODO
impl Anchor {
    pub fn from(_path: Symbol, _lineno: usize, _offset: usize, _buffer: usize) -> Anchor {
        return Anchor{path: _path, lineno: _lineno, offset: _offset, buffer: _buffer}
    }
    pub fn is_boring(&self, map: &mut SymbolMap) -> bool {
        return *self == builtin_anchor(map) || *self == unknown_anchor(map);
    }
    pub fn is_same(&self, other: &mut Anchor) -> bool {
        return self == other
    }
}

fn builtin_anchor(map: &mut SymbolMap) -> Anchor {
    return Anchor::from(SymbolMap::add_symbol(map, String::from("builtin")), 1, 1, 0);
}

fn unknown_anchor(map: &mut SymbolMap) -> Anchor {
    return Anchor::from(SymbolMap::add_symbol(map, String::from("builtin")), 1, 1, 0);
}