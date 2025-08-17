use super::super::core::position::{Position, Span};
use super::super::errors::{RenderError};
use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct Assert {
    pub condition: Box<Expression>,
    pub message: Option<Box<Expression>>,
    pub span: Span,
}

impl Assert {
    pub fn new_span(condition: Expression, message: Option<Expression>, span: Span) -> Self {
        Self {
            condition: Box::new(condition),
            message: message.map(Box::new),
            span,
        }
    }

    pub fn new(condition: Expression, message: Option<Expression>) -> Self {
        Self::new_span(
            condition,
            message,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let condition = self.condition.render()?;
        let message = match &self.message {
            Some(msg) => format!(" with message {}", msg.render()?),
            None => String::new(),
        };
        Ok(format!("assert {}{}", condition, message))
    }
}

#[derive(Clone, Debug)]
pub struct Throw {
    pub message: Box<Expression>,
    pub span: Span,
}

impl Throw {
    pub fn new_span(message: Expression, span: Span) -> Self {
        Self {
            message: Box::new(message),
            span,
        }
    }

    pub fn new(message: Expression) -> Self {
        Self::new_span(
            message,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        Ok(format!("throw {}", self.message.render()?))
    }
}