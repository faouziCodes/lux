use luxast::span::Span;

pub enum ParseErrorTypes {
    MissingField,
    UnextpectedToken,
}

pub trait ParseError<T> {
    fn loc(&self) -> Span;
    fn msg(&self);
    fn error_type(&self) -> ParseErrorTypes;
}
