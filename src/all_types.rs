use crate::qualify_type::QualifyType;



//To get rid of isa
pub enum All_types<'a> {
    //TODO
    qualify_type(&'a QualifyType<'a>),
    
}