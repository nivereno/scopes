use crate::types::{Type, B_Types};


pub struct FunctionType<'a> {
    this: &'a Type,
    except_type: &'a Type,
    return_type: &'a Type,
    argument_types: Vec<&'a Type>,
    flags: u32,
    stripped: &'a mut FunctionType<'a>
}
const FF_VARIADIC: u32 = 1 << 0;
impl <'a>FunctionType<'a> {

    fn strip_annotations(&self) -> &'a FunctionType {

        todo!()
    }
    fn has_exception(&self, types: &B_Types) -> bool {
        return self.except_type != &types.TYPE_NoReturn.this
    }
    fn vararg(&self) -> bool {
        return self.flags & FF_VARIADIC != 0
    }
    fn type_at_index(i: usize) -> Option<&'a Type> {
        todo!()
    }

}
pub fn canonicalize_unique_types() {

}
pub fn canonicalize_argument_types() {

}
pub fn raising_function_type<'a>() -> &'a Type {
    todo!()
}
pub fn raising_function_type_123<'a>() -> &'a Type {
    //overload
    todo!()
}
pub fn function_type<'a>() -> &'a Type {
    todo!()
}
pub fn is_function_pointer() -> bool {
    todo!()
}
pub fn extract_function_type<'a>() -> &'a FunctionType<'a> {
    todo!()
}
pub fn varify_function_pointer() -> Result<(), anyhow::Error> {
    todo!()
}
