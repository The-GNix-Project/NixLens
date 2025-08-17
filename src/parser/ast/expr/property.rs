use super::super::core::position::{Position, Span};
use super::super::errors::RenderError;
use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct PropertyAccess {
    pub expression: Box<Expression>,
    pub attribute_path: Vec<Expression>,
    pub default: Option<Box<Expression>>,
    pub span: Span,
}

impl PropertyAccess {
    pub fn new_span(
        expression: Expression,
        attribute_path: Vec<Expression>,
        default: Option<Expression>,
        span: Span,
    ) -> Self {
        Self {
            expression: Box::new(expression),
            attribute_path,
            default: default.map(Box::new),
            span,
        }
    }

    pub fn new(
        expression: Expression,
        attribute_path: Vec<Expression>,
        default: Option<Expression>,
    ) -> Self {
        Self::new_span(
            expression,
            attribute_path,
            default,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let path = self
            .attribute_path
            .iter()
            .map(|e| e.render())
            .collect::<Result<Vec<_>, _>>()?
            .join(".");
        match &self.default {
            Some(default) => Ok(format!(
                "{}.?{}.or({})",
                self.expression.render()?,
                path,
                default.render()?
            )),
            None => Ok(format!("{}.{}", self.expression.render()?, path)),
        }
    }

    pub fn debug(&self) -> String {
        format!(
            "PropertyAccess(expr={:?}, path={:?}, default={:?})",
            self.expression, self.attribute_path, self.default
        )
    }
}