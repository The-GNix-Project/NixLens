use super::super::core::position::{Position, Span};
use super::super::errors::{RenderError};
use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct List {
    pub elements: Vec<Expression>,
    pub span: Span,
}

impl List {
    pub fn new_span(elements: Vec<Expression>, span: Span) -> Self {
        Self { elements, span }
    }

    pub fn new(elements: Vec<Expression>) -> Self {
        Self::new_span(
            elements,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let elems = self
            .elements
            .iter()
            .map(|e| e.render())
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        Ok(format!("[{}]", elems))
    }

    pub fn debug(&self) -> String {
        format!("List({:?})", self.elements)
    }
}

#[derive(Clone, Debug)]
pub struct  AttrSet {
    pub recursive: bool,
    pub bindings: Vec<Expression>,
    pub span: Span,
}

impl AttrSet {
    pub fn new_span(recursive: bool, bindings: Vec<Expression>, span: Span) -> Self {
        Self {
            recursive,
            bindings,
            span,
        }
    }

    pub fn new(recursive: bool, bindings: Vec<Expression>) -> Self {
        Self::new_span(
            recursive,
            bindings,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let bindings = self
            .bindings
            .iter()
            .map(|b| b.render())
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        Ok(if self.recursive {
            format!("rec {{ {} }}", bindings)
        } else {
            format!("{{ {} }}", bindings)
        })
    }

    pub fn debug(&self) -> String {
        format!(" AttrSet(recursive={}, {:?})", self.recursive, self.bindings)
    }
}