mod node;
pub mod span;
pub mod types;
pub mod context;

use node::Node;
use span::Span;

pub struct Ast {
    entry: Node,
    location: Span,
}

impl Ast {
    fn new(location: Span) -> Self {
        Self {
            entry: Node::default(),
            location,
        }
    }
}
