use crate::{Anchor, symbol::Symbol};

//#[derive(Clone)]
//pub struct Value<T: Sized>(pub T);
#[derive(PartialEq, Clone)]
pub enum Value {
    i8(i8),
    i16(i16),
    i32(i32),
    i64(i64),
    u8(u8),
    u16(u16),
    u32(u32),
    u64(u64),
    char(char),
    isize(isize),
    usize(usize),
    f32(f32),
    f64(f64),
    string(String),
    symbol(Symbol),
    None
}
#[derive(PartialEq, Clone)]
pub struct ValueRef {
    pub value: Value,
    pub anchor: Anchor::Anchor,
}
impl Value {
    pub fn anchor(&self) {

    }
    /*pub fn get_value(self) -> ret {
        match self {
            Value::i8(i) =>  return i,


        }
    }*/
}
/*
pub trait ValueRef {
}

impl<T: Sized> ValueRef for Value<T> {
    
}*/

