use crate::types::type_builder::TypeBuilder;

pub enum FloatTypes {
    UF64,
    UF32,
    UF8,
    F8,
    F32,
    F64,
}

pub struct FloatType {
    float_t: FloatTypes,
    is_pointer: bool,
}

impl FloatType {
    pub fn new(float_t: FloatTypes, is_pointer: bool) -> Self {
       Self {
           float_t,
           is_pointer
       }
    }
}

impl TypeBuilder<FloatTypes> for FloatType {
    fn set_ptr(&mut self) {
        self.is_pointer  = true;
    }

    fn cast(&mut self, to: FloatTypes) {
        self.float_t = to;
    }
}
