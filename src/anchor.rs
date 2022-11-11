use crate::symbol::Symbol;


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
}