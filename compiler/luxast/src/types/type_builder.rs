pub trait TypeBuilder<Cast> {
    /// Turns the type into a pointer type
    fn set_ptr(&mut self);
    /// Casts type into Cast T 
    fn cast(&mut self, to: Cast);
}
