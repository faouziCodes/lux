#[derive(Clone, Copy)]
pub struct Span {
    line: usize,
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Returns a new span, where start, and end will be the argument passed;
    pub fn new(line: usize, pos: usize) -> Self {
        Self {
            line,
            start: pos,
            end: pos,
        }
    }

    pub fn set_start(&mut self, pos: usize) {
        self.start = pos;
    }

    pub fn set_end(&mut self, pos: usize) {
        self.end = pos;
    }

    pub fn set_line(&mut self, pos: usize) {
        self.line = pos;
    }
}
