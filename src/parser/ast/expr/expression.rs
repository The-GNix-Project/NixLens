use super::super::core::{Boolean, Float, Identifier, Integer, LiteralString, Null, Path};
use super::operations::BinaryOperation;
use super::bindings::{BindingInherit, BindingKeyValue, LetIn, With};
use super::collection::{List, AttrSet};
use super::control::IfThenElse;
use super::function::{Function, FunctionApplication};
use super::property::PropertyAccess;
use super::string::{IndentedString, NixString};
use super::nix_errors::{Assert, Throw};

use super::super::errors::RenderError;

#[derive(Clone, Debug)]
pub enum Expression {
    Integer(Integer),
    Float(Float),
    Identifier(Identifier),
    LiteralString(LiteralString),
    Boolean(Boolean),
    Null(Null),
    Path(Path),
    BinaryOperation(BinaryOperation),
    List(List),
    AttrSet(AttrSet),
    IfThenElse(IfThenElse),
    Function(Function),
    FunctionApplication(FunctionApplication),
    PropertyAccess(PropertyAccess),
    NixString(NixString),
    IndentedString(IndentedString),
    BindingInherit(BindingInherit),
    BindingKeyValue(BindingKeyValue),
    With(With),
    LetIn(LetIn),
    Assert(Assert),
    Throw(Throw),
}

impl Expression {
    pub fn render(&self) -> Result<String, RenderError> {
        match self {
            Expression::Integer(x) => Ok(x.render()),
            Expression::Float(x) => Ok(x.render()),
            Expression::Identifier(x) => Ok(x.render()),
            Expression::LiteralString(x) => Ok(x.render()),
            Expression::Boolean(x) => Ok(x.render()),
            Expression::Null(x) => Ok(x.render()),
            Expression::Path(x) => Ok(x.render()),
            Expression::BinaryOperation(x) => x.render(),
            Expression::AttrSet(x) => x.render(),
            Expression::List(x) => x.render(),
            Expression::IfThenElse(x) => x.render(),
            Expression::Function(x) => x.render(),
            Expression::FunctionApplication(x) => x.render(),
            Expression::PropertyAccess(x) => x.render(),
            Expression::NixString(x) => x.render(),
            Expression::IndentedString(x) => x.render(),
            Expression::With(x) => x.render(),
            Expression::BindingInherit(x) => x.render(),
            Expression::BindingKeyValue(x) => x.render(),
            Expression::LetIn(x) => x.render(),
            Expression::Assert(x) => x.render(),
            Expression::Throw(x) => x.render(),
        }
    }
}
