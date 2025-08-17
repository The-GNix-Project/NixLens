use super::position::{Position, Span};

#[derive(Clone, Debug)]
pub struct Identifier {
    pub id: String,
    pub span: Span,
}

impl Identifier {
    pub fn new_span(id: String, span: Span) -> Self {
        Self { id, span }
    }

    pub fn new(id: String) -> Self {
        Self {
            id,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn debug(&self) -> String {
        format!("{}", self.id)
    }

    pub fn render(&self) -> String {
        format!("{}", self.id)
    }
}