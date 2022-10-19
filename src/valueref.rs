#[derive(Clone)]
pub struct Value<T: Sized + Clone + From<T>>(pub T);

pub trait ValueRef {
}

impl<T: Sized + Clone + From<T>> ValueRef for Value<T> {
}

impl dyn ValueRef {
    pub fn anchor(&self) {

    }
}