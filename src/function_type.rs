use crate::types::{Type, B_Types};
use anyhow::{anyhow, Result, Ok};

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
    fn type_at_index(&self, i: usize) -> Result<&'a Type, anyhow::Error> {
        if self.argument_types.len() > i {
            return Ok(self.argument_types[i]);
        }
        return Err(anyhow!("Index out of range in argument_types"))
    }

}
pub fn canonicalize_unique_types() {

}
pub fn canonicalize_argument_types() {

}
pub fn raising_function_type<'a>(except_type: &Type, return_type: &Type, argument_types: &Vec<&Type>, flags: u32) -> &'a Type {
    todo!()
}
pub fn raising_function_type_default<'a>(types: B_Types, return_type: &Type, argument_types: &Vec<&Type>, flags: u32) -> &'a Type {
    //overload
    return raising_function_type(&types.TYPE_Error.this, return_type, argument_types, flags)
}
pub fn function_type<'a>(types: B_Types, return_type: &Type, argument_types: &Vec<&Type>, flags: u32) -> &'a Type {
    return raising_function_type(&types.TYPE_NoReturn.this, return_type, argument_types, flags)
}
pub fn is_function_pointer(t: &Type) -> bool {
    todo!()
}
pub fn extract_function_type<'a>() -> &'a FunctionType<'a> {
    todo!()
}
pub fn varify_function_pointer(t: &Type) -> Result<(), anyhow::Error> {
    if !is_function_pointer(t) {
        return Err(anyhow!("FunctionPointerExpected"));
    }
    return Ok(())
}
