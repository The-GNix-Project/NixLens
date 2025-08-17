use super::super::core::position::{Position, Span};
use super::super::errors::RenderError;
use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct IfThenElse {
    pub predicate: Box<Expression>,
    pub then: Box<Expression>,
    pub else_: Box<Expression>,
    pub span: Span,
}

impl IfThenElse {
    pub fn new_span(
        predicate: Expression,
        then: Expression,
        else_: Expression,
        span: Span,
    ) -> Self {
        Self {
            predicate: Box::new(predicate),
            then: Box::new(then),
            else_: Box::new(else_),
            span,
        }
    }

    pub fn new(predicate: Expression, then: Expression, else_: Expression) -> Self {
        Self::new_span(
            predicate,
            then,
            else_,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        Ok(format!(
            "if {} then {} else {}",
            self.predicate.render()?,
            self.then.render()?,
            self.else_.render()?
        ))
    }

    pub fn debug(&self) -> String {
        format!(
            "IfThenElse({:?}, {:?}, {:?})",
            self.predicate, self.then, self.else_
        )
    }
}