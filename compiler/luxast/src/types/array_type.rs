use crate::types::type_builder::TypeBuilder;
use super::{float_type::FloatType, int_type::IntType};

pub enum ArrayTypes {
    Int(IntType),
    Float(FloatType),
}

pub struct ArrayType {
    size: usize,
    array_type: ArrayTypes,
    is_pointer: bool
}

impl ArrayType {
    pub fn new(array_type: ArrayTypes, size: usize, is_pointer: bool) -> Self {
        Self {
            size,
            array_type,
            is_pointer
        }
    }
}

impl TypeBuilder<ArrayTypes> for ArrayType {
    fn set_ptr(&mut self) {
        self.is_pointer = true;
    }

    fn cast(&mut self, to: ArrayTypes) {
        self.array_type = to;
    }
}
