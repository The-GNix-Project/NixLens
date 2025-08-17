use super::super::core::operators::Operator;
use super::super::core::position::{Position, Span};
use super::super::errors::{RenderError, SyntaxError};
use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
    pub span: Span,
}

impl BinaryOperation {
    pub fn new_span(
        left: Expression,
        operator: Operator,
        right: Expression,
        span: Span,
    ) -> Result<Self, SyntaxError> {
        if !operator.is_binary() {
            return Err(SyntaxError {
                message: format!("Tried to create binary operation using {:?}", operator),
            });
        }
        Ok(Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            span,
        })
    }

    pub fn new(
        left: Expression,
        operator: Operator,
        right: Expression,
    ) -> Result<Self, SyntaxError> {
        Self::new_span(
            left,
            operator,
            right,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        Ok(format!(
            "({} {} {})",
            self.left.render()?,
            self.operator.render(),
            self.right.render()?
        ))
    }

    pub fn debug(&self) -> String {
        format!(
            "BinaryOperation({:?}, {:?}, {:?})",
            self.left, self.operator, self.right
        )
    }
}

#[derive(Clone, Debug)]
pub struct UnaryOperation {
    pub operator: Operator,
    pub expression: Box<Expression>,
    pub span: Span,
}

impl UnaryOperation {
    pub fn new_span(
        operator: Operator,
        expression: Expression,
        span: Span,
    ) -> Result<Self, SyntaxError> {
        if !operator.is_unary() {
            return Err(SyntaxError {
                message: format!("Tried to create unary operation with {:?}", operator),
            });
        }
        Ok(Self {
            operator,
            expression: Box::new(expression),
            span,
        })
    }

    pub fn new(operator: Operator, expression: Expression) -> Result<Self, SyntaxError> {
        Self::new_span(
            operator,
            expression,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        Ok(format!(
            "{}{}",
            self.operator.render(),
            self.expression.render()?
        ))
    }

    pub fn debug(&self) -> String {
        format!("UnaryOperation({:?}, {:?})", self.operator, self.expression)
    }
}
