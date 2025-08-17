use super::super::core::position::{Position, Span};
use super::super::errors::RenderError;
use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct NixString {
    pub parts: Vec<Expression>,
    pub span: Span,
}

impl NixString {
    pub fn new_span(parts: Vec<Expression>, span: Span) -> Self {
        Self { parts, span }
    }

    pub fn new(parts: Vec<Expression>) -> Self {
        Self::new_span(parts, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn render(&self) -> Result<String, RenderError> {
        Ok(self
            .parts
            .iter()
            .map(|p| p.render())
            .collect::<Result<Vec<_>, _>>()?
            .join(""))
    }

    pub fn debug(&self) -> String {
        format!("String({:?})", self.parts)
    }
}

#[derive(Clone, Debug)]
pub struct IndentedString {
    pub parts: Vec<Expression>,
    pub span: Span,
}

impl IndentedString {
    pub fn new_span(parts: Vec<Expression>, span: Span) -> Self {
        Self { parts, span }
    }

    pub fn new(parts: Vec<Expression>) -> Self {
        Self::new_span(parts, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn render(&self) -> Result<String, RenderError> {
        Ok(self
            .parts
            .iter()
            .map(|p| p.render())
            .collect::<Result<Vec<_>, _>>()?
            .join(""))
    }

    pub fn debug(&self) -> String {
        format!("IndentedString({:?})", self.parts)
    }
}

#[derive(Clone, Debug)]
pub struct PartInterpolation {
    pub expression: Box<Expression>,
    pub span: Span,
}

impl PartInterpolation {
    pub fn new_span(expression: Expression, span: Span) -> Self {
        Self {
            expression: Box::new(expression),
            span,
        }
    }

    pub fn new(expression: Expression, span: Span) -> Self {
        Self {
            expression: Box::new(expression),
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let expression = self.expression.render()?;
        Ok(format!("${{{}}}", expression))
    }

    pub fn debug(&self) -> String {
        format!("PartInterpolation({:?})", self.expression)
    }
}