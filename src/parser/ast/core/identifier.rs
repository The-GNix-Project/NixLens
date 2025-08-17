use super::position::{Position, Span};
use super::super::super::parser::Rule;

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

    pub fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        Self::new_span(pair.as_str().to_string(), Span::from_pest_span(pair.as_span()))
    }

}