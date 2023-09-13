use crate::types::type_builder::TypeBuilder;

pub enum IntTypes {
    U64,
    U32,
    U8,
    I8,
    I32,
    I64,
}

pub struct IntType {
    int_type: IntTypes,
    is_pointer: bool,
}

impl IntType {
    pub fn new(int_type: IntTypes, is_pointer: bool) -> Self {
        Self {
            int_type,
            is_pointer,
        }
    }
}

impl TypeBuilder<IntTypes> for IntType {
    fn set_ptr(&mut self) {
        self.is_pointer = true;
    }

    fn cast(&mut self, to: IntTypes) {
        self.int_type = to;
    }
}
