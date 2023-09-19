use luxast::span::Span;

pub struct ErrorMsg {
    msg: String,
}

pub enum ParseErrorTypes {
    MissingField,
    UnextpectedToken,
}

pub trait ParseError<T> {
    fn loc(&self) -> Span;
    fn msg(&self);
}
