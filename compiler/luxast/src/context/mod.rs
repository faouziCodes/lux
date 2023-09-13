use crate::types::{
    array_type::ArrayType,
    float_type::{FloatType, FloatTypes},
    int_type::IntType,
};
use crate::types::array_type::ArrayTypes;

pub struct Context;

impl Context {
    fn int_type(int_type: IntTypes, is_pointer: bool) -> IntType {
        IntType::new(is_pointer, int_type);
    }

    fn float_type(float_type: FloatTypes, is_pointer: bool) -> FloatType {
        FloatType::new(float_type, is_pointer)
    }

    fn array_type(array_type: ArrayTypes, is_pointer: bool, size: usize) -> ArrayType {
        ArrayType::new(array_type, is_pointer, size)
    }
}