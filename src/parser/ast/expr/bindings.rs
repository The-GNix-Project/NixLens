use super::super::core::position::{Position, Span};
use super::super::errors::RenderError;
use super::super::expr::expression::Expression;

#[derive(Clone, Debug)]
pub struct LetIn {
    pub bindings: Vec<Expression>,
    pub target: Box<Expression>,
    pub span: Span,
}

impl LetIn {
    pub fn new_span(bindings: Vec<Expression>, target: Expression, span: Span) -> Self {
        Self {
            bindings,
            target: Box::new(target),
            span,
        }
    }

    pub fn new(bindings: Vec<Expression>, target: Expression) -> Self {
        Self::new_span(
            bindings,
            target,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let bindings_str = self
            .bindings
            .iter()
            .map(|b| b.render())
            .collect::<Result<Vec<_>, _>>()?
            .join(" ");
        Ok(format!("let {} in {}", bindings_str, self.target.render()?))
    }
}

#[derive(Clone, Debug)]
pub struct BindingInherit {
    pub from_: Option<Box<Expression>>,
    pub attributes: Box<Expression>,
    pub span: Span,
}

impl BindingInherit {
    pub fn new_span(from_: Option<Expression>, attributes: Expression, span: Span) -> Self {
        Self {
            from_: from_.map(Box::new),
            attributes: Box::new(attributes),
            span,
        }
    }

    pub fn new(from_: Option<Expression>, attributes: Expression) -> Self {
        Self::new_span(
            from_,
            attributes,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        match &self.from_ {
            Some(from_expr) => Ok(format!(
                "inherit ( {}) {}",
                from_expr.render()?,
                self.attributes.render()?
            )),
            None => Ok(format!("inherit {}", self.attributes.render()?)),
        }
    }

    pub fn debug(&self) -> String {
        format!("BindingInherit(from={})", self.from_.is_some())
    }
}

// MARK: BindingKeyValue
#[derive(Clone, Debug)]
pub struct BindingKeyValue {
    pub from_: Box<Expression>,
    pub to: Box<Expression>,
}

impl BindingKeyValue {
    pub fn new_span(from_: Expression, to: Expression) -> Self {
        Self {
            from_: Box::new(from_),
            to: Box::new(to),
        }
    }

    pub fn new(from_: Expression, to: Expression) -> Self {
        Self::new_span(from_, to)
    }

    pub fn render(&self) -> Result<String, RenderError> {
        Ok(format!("{} = {};", self.from_.render()?, self.to.render()?))
    }

    pub fn debug(&self) -> String {
        format!("KeyValue({:?})", self.from_)
    }
}

#[derive(Clone, Debug)]
pub struct With {
    pub scope: Box<Expression>,
    pub body: Box<Expression>,
    pub span: Span,
}

impl With {
    pub fn new_span(scope: Expression, body: Expression, span: Span) -> Self {
        Self {
            scope: Box::new(scope),
            body: Box::new(body),
            span,
        }
    }

    pub fn new(scope: Expression, body: Expression) -> Self {
        Self::new_span(
            scope,
            body,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        Ok(format!("with {}; {}", self.scope.render()?, self.body.render()?))
    }
}