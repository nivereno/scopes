
pub struct Value<T: Sized>(pub T);

pub trait ValueRef {
}

impl<T: Sized> ValueRef for Value<T> {
}

impl dyn ValueRef {
    pub fn anchor(&self) {

    }
}