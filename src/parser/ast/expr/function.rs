use super::super::core::position::{Position, Span};
use super::super::errors::RenderError;
use super::super::core::identifier::Identifier;
use super::expression::Expression;  

#[derive(Clone, Debug)]
pub struct FunctionHeadDestructuredArgument {
    pub identifier: String,
    pub default: Option<Expression>,
}

impl FunctionHeadDestructuredArgument {
    pub fn new(identifier: String, default: Option<Expression>) -> Self {
        Self {
            identifier,
            default,
        }
    }

    pub fn debug(&self) -> String {
        format!(
            "FunctionHeadDestructuredArgument(identifier='{}', default={:?})",
            self.identifier, self.default
        )
    }

    pub fn render(&self) -> Result<String, RenderError> {
        match &self.default {
            Some(expr) => {
                let rendered = expr.render()?; // unwraps Ok or returns Err
                Ok(format!("{} ? {}", self.identifier, rendered))
            }
            None => Ok(self.identifier.clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionHeadDestructured {
    pub ellipsis: bool,
    pub identifier: Identifier,
    pub arguments: Vec<FunctionHeadDestructuredArgument>,
    pub span: Span,
}

impl FunctionHeadDestructured {
    pub fn new_span(
        ellipsis: bool,
        identifier: Identifier,
        arguments: Vec<FunctionHeadDestructuredArgument>,
        span: Span,
    ) -> Self {
        Self {
            ellipsis,
            identifier,
            arguments,
            span,
        }
    }

    pub fn new(
        ellipsis: bool,
        identifier: Identifier,
        arguments: Vec<FunctionHeadDestructuredArgument>,
    ) -> Self {
        Self {
            ellipsis,
            identifier,
            arguments,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn debug(&self) -> String {
        format!("FunctionHeadDestructured(ellipsis={})", self.ellipsis)
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let mut result: String = String::new();
        for argument in &self.arguments {
            result.push_str(&argument.render()?)
        }
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionHeadSimple {
    pub identifier: Identifier,
    pub span: Span,
}

impl FunctionHeadSimple {
    pub fn new_span(identifier: Identifier, span: Span) -> Self {
        Self { identifier, span }
    }

    pub fn new(identifier: Identifier) -> Self {
        Self {
            identifier,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn render(&self) -> String {
        self.identifier.render()
    }

    pub fn debug(&self) -> String {
        format!(
            "FunctionHeadSimple(identifier={:?}, span={:?})",
            self.identifier, self.span
        )
    }
}

pub enum FunctionHead {
    FunctionHeadSimple(FunctionHeadSimple),
    FunctionHeadDestructured(FunctionHeadDestructured),
}

impl FunctionHead {
    pub fn render(&self) -> Result<String, RenderError> {
        match self {
            FunctionHead::FunctionHeadSimple(f) => Ok(f.render()),
            FunctionHead::FunctionHeadDestructured(f) => f.render(),
        }
    }

    pub fn debug(&self) -> String {
        match self {
            FunctionHead::FunctionHeadSimple(x) => x.debug(),
            FunctionHead::FunctionHeadDestructured(x) => x.debug(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub head: Box<Expression>,
    pub body: Box<Expression>,
    pub span: Span,
}

impl Function {
    pub fn new_span(head: Expression, body: Expression, span: Span) -> Self {
        Self {
            head: Box::new(head),
            body: Box::new(body),
            span,
        }
    }

    pub fn new(head: Expression, body: Expression) -> Self {
        Self {
            head: Box::new(head),
            body: Box::new(body),
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let head = self.head.render()?;
        let body = self.body.render()?;
        Ok(format!("{}: {}", head, body))
    }

    pub fn debug(&self) -> String {
        format!("Function({:?}, {:?})", self.head, self.body)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionApplication {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

impl FunctionApplication {
    pub fn new_span(function: Expression, arguments: Vec<Expression>, span: Span) -> Self {
        Self {
            function: Box::new(function),
            arguments,
            span,
        }
    }

    pub fn new(function: Expression, arguments: Vec<Expression>) -> Self {
        Self {
            function: Box::new(function),
            arguments,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn render(&self) -> Result<String, RenderError> {
        let args = self
            .arguments
            .iter()
            .map(|a| a.render())
            .collect::<Result<Vec<_>, _>>()?;
        let function = self.function.render()?;
        Ok(format!("{} {}", function, args.join(" ")))
    }

    pub fn debug(&self) -> String {
        format!(
            "FunctionApplication(function={:?}, arguments={:?})",
            self.function, self.arguments
        )
    }
}
