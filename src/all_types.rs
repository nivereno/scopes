use crate::qualify_type::{QualifyType, Qualifier};



//To get rid of isa
pub enum All_types<'a> {
    //TODO
    qualify_type(&'a QualifyType<'a>),
    qualifier(&'a Qualifier)
}

impl <'a>All_types<'a> {
    //TODO return an enum maybe
    pub fn kind(&self) -> usize {
        match self {
            All_types::qualify_type(T) => return T.kind(),
            All_types::qualifier(T) => return T.kind(),
        }
    }
}