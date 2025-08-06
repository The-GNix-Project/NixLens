// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of GNix.
// GNix - The Graphical Nix Project
// -----------------------------------------------------------------------------------------|
// GNix is free software: you can redistribute it and/or modify                             |
// it under the terms of the GNU General Public License as published by                     |
// the Free Software Foundation, either version 3 of the License, or any later version.     |
//                                                                                          |
// GNix is distributed in the hope that it will be useful,                                  |
// but WITHOUT ANY WARRANTY; without even the implied warranty of                           |
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                            |
// GNU General Public License for more details.                                             |
//                                                                                          |
// You should have received a copy of the GNU General Public License                        |
// along with GNix.  If not, see <https://www.gnu.org/licenses/>.                           |
// -----------------------------------------------------------------------------------------|

#![allow(dead_code, unused_variables, unused_imports)]

use std::fmt;

// ==================== CORE STRUCTURES =================
// MARK: Position
#[derive(Clone, Debug)]
pub struct Position {
    pub line: i64,
    pub column: i64,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position({}, {})", self.line, self.column)
    }
}

impl Position {
    pub fn new(line: i64, column: i64) -> Self {
        Self { line, column }
    }

    pub fn debug(&self) -> String {
        format!("{}", self)
    }
}

// MARK: Span
#[derive(Clone, Debug)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Span({}, {})", self.start, self.end)
    }
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn debug(&self) -> String {
        format!("{}", self)
    }
}

// MARK: Identifier
#[derive(Clone, Debug)]
pub struct Identifier {
    pub id: String,
    pub span: Span,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier({})", self.id)
    }
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
}

// MARK: Error
#[derive(Clone, Debug)]
pub struct Error {
    pub message: String,
    pub span: Span,
}

impl Error {
    pub fn new_span(message: String, span: Span) -> Self {
        Self { message, span }
    }

    pub fn new(message: String) -> Self {
        Self {
            message,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn debug(&self) -> String {
        format!("Error('{}')", self.message)
    }

    //pub fn render(&self) -> Result<String, Box<dyn std::error::Error>> {  // TODO: choose the best way of handling this
    //    Err(Box::new_span(self.clone()))
    //}
}

// MARK: Float
#[derive(Clone, Debug)]
pub struct Float {
    pub value: String,
    pub span: Span,
}

impl Float {
    pub fn new_span(value: String, span: Span) -> Self {
        Self { value, span }
    }

    pub fn new(value: String) -> Self {
        Self {
            value,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn debug(&self) -> String {
        format!("Float('{}')", self.value)
    }

    pub fn render(&self) -> String {
        format!("{}", self.value)
    }
}

// MARK: Integer
#[derive(Clone, Debug)]
pub struct Integer {
    pub value: String,
    pub span: Span,
}

impl Integer {
    pub fn new_span(value: String, span: Span) -> Self {
        Self { value, span }
    }

    pub fn new(value: String) -> Self {
        Self {
            value,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn debug(&self) -> String {
        format!("Integer('{}')", self.value)
    }

    pub fn render(&self) -> String {
        format!("{}", self.value)
    }
}

// ==================== OPERATORS ====================
// MARK: OPERATORS
macro_rules! impl_operator {
    ($( $name:ident => $render:expr ),+ $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name;

            impl $name {
                pub fn value() -> &'static str {
                    stringify!($name)
                }

                pub fn debug(&self) -> String {
                    format!("{}", stringify!($name))
                }

                pub fn render(&self) -> String {
                    $render.to_string()
                }
            }
        )+
    };
}

impl_operator!(
    Addition => "+",
    Subtraction => "-",
    Multiplication => "*",
    Division => "/",
    EqualTo => "==",
    NotEqualTo => "!=",
    GreaterThan => ">",
    GreaterThanOrEqualTo => ">=",
    LessThan => "<",
    LessThanOrEqualTo => "<=",
    LogicalAnd => "&&",
    LogicalOr => "||",
    Not => "!",
    Negate => "-",
    Concatenation => "++",
    Implication => "=>",
    Update => ":=",
);

#[derive(Clone, Debug)]
pub enum Operator {
    Addition(Addition),
    Concatenation(Concatenation),
    EqualTo(EqualTo),
    GreaterThan(GreaterThan),
    GreaterThanOrEqualTo(GreaterThanOrEqualTo),
    Division(Division),
    Implication(Implication),
    LessThan(LessThan),
    LessThanOrEqualTo(LessThanOrEqualTo),
    LogicalAnd(LogicalAnd),
    LogicalOr(LogicalOr),
    Multiplication(Multiplication),
    NotEqualTo(NotEqualTo),
    Subtraction(Subtraction),
    Update(Update),
    Not(Not),
    Negate(Negate),
}

impl Operator {
    pub fn render(&self) -> String {
        match self {
            Operator::Addition(x) => x.render(),
            Operator::Concatenation(x) => x.render(),
            Operator::EqualTo(x) => x.render(),
            Operator::GreaterThan(x) => x.render(),
            Operator::GreaterThanOrEqualTo(x) => x.render(),
            Operator::Division(x) => x.render(),
            Operator::Implication(x) => x.render(),
            Operator::LessThan(x) => x.render(),
            Operator::LessThanOrEqualTo(x) => x.render(),
            Operator::LogicalAnd(x) => x.render(),
            Operator::LogicalOr(x) => x.render(),
            Operator::Multiplication(x) => x.render(),
            Operator::NotEqualTo(x) => x.render(),
            Operator::Subtraction(x) => x.render(),
            Operator::Update(x) => x.render(),
            Operator::Not(x) => x.render(),
            Operator::Negate(x) => x.render(),
        }
    }
}

// ==================== FUNCTION STRUCTURES ====================
// MARK: FunctionHeadDestructuredArgument
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

    pub fn render(&self) -> String {
        match &self.default {
            Some(expr) => format!("{} ? {:?}", self.identifier, expr.render()),
            None => self.identifier.clone(),
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

    pub fn render(&self) -> String {
        let mut result: String = String::new();
        for argument in &self.arguments {
            result.push_str(&argument.render())
        }
        result
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

    pub fn debug(&self) -> String {
        format!(
            "FunctionHeadSimple(identifier={}, span={})",
            self.identifier, self.span
        )
    }
}

enum FunctionHead {
    FunctionHeadSimple(FunctionHeadSimple),
    FunctionHeadDestructured(FunctionHeadDestructured),
}

impl FunctionHead {
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

    pub fn new(function: Expression, arguments: Vec<Expression>, span: Span) -> Self {
        Self {
            function: Box::new(function),
            arguments,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn debug(&self) -> String {
        format!(
            "FunctionApplication(function={:?}, arguments={:?})",
            self.function, self.arguments
        )
    }
}

// ==================== PARTS ====================
// MARK: PartInterpolation
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

    pub fn debug(&self) -> String {
        format!("PartInterpolation({:?})", self.expression)
    }
}

// MARK: PartRaw
#[derive(Clone, Debug)]
pub struct PartRaw {
    pub content: String,
    pub span: Span,
}

impl PartRaw {
    pub fn new_span(content: String, span: Span) -> Self {
        Self { content, span }
    }

    pub fn new(content: String) -> Self {
        Self {
            content,
            span: Span::new(Position::new(1, 1), Position::new(1, 1)),
        }
    }

    pub fn debug(&self) -> String {
        format!("PartRaw('{}')", self.content)
    }
}

// ==================== EXPRESSIONS ====================
// MARK: BinaryOperation
#[derive(Clone, Debug)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
    pub span: Span,
}

impl BinaryOperation {
    pub fn new_span(left: Expression, operator: Operator, right: Expression, span: Span) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            span,
        }
    }

    pub fn new(left: Expression, operator: Operator, right: Expression) -> Self {
        Self::new_span(
            left,
            operator,
            right,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn debug(&self) -> String {
        format!(
            "BinaryOperation({:?}, {:?}, {:?})",
            self.left, self.operator, self.right
        )
    }
}

// MARK: Assert
#[derive(Clone, Debug)]
pub struct Assert {
    pub expression: Box<Expression>,
    pub target: Box<Expression>,
    pub span: Span,
}

impl Assert {
    pub fn new_span(expression: Expression, target: Expression, span: Span) -> Self {
        Self {
            expression: Box::new(expression),
            target: Box::new(target),
            span,
        }
    }

    pub fn new(expression: Expression, target: Expression) -> Self {
        Self::new_span(
            expression,
            target,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn debug(&self) -> String {
        format!(
            "Assert(expr={:?}, target={:?})",
            self.expression, self.target
        )
    }
}

// ==================== EXPRESSIONS ====================
// MARK: HasAttribute
#[derive(Clone, Debug)]
pub struct HasAttribute {
    pub expression: Box<Expression>,
    pub attribute_path: Vec<Expression>,
    pub span: Span,
}

impl HasAttribute {
    pub fn new_span(expression: Expression, attribute_path: Vec<Expression>, span: Span) -> Self {
        Self {
            expression: Box::new(expression),
            attribute_path,
            span,
        }
    }

    pub fn new(expression: Expression, attribute_path: Vec<Expression>) -> Self {
        Self::new_span(
            expression,
            attribute_path,
            Span::new(Position::new(1, 1), Position::new(1, 1)),
        )
    }

    pub fn debug(&self) -> String {
        format!("HasAttribute({:?})", self.attribute_path)
    }
}

// MARK: IndentedString
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

    pub fn debug(&self) -> String {
        format!("IndentedString({:?})", self.parts)
    }
}

// MARK: IfThenElse
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

    pub fn debug(&self) -> String {
        format!(
            "IfThenElse({:?}, {:?}, {:?})",
            self.predicate, self.then, self.else_
        )
    }
}

// MARK: LetIn
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

    pub fn debug(&self) -> String {
        format!("LetIn({:?})", self.bindings)
    }
}

// ==================== COLLECTIONS ====================
// MARK: List
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

    pub fn debug(&self) -> String {
        format!("List({:?})", self.elements)
    }
}

// MARK: Map
#[derive(Clone, Debug)]
pub struct Map {
    pub recursive: bool,
    pub bindings: Vec<Expression>,
    pub span: Span,
}

impl Map {
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

    pub fn debug(&self) -> String {
        format!("Map(recursive={}, {:?})", self.recursive, self.bindings)
    }
}

// ==================== PATH & URI ====================
// MARK: Path
#[derive(Clone, Debug)]
pub struct Path {
    pub parts: Vec<Expression>,
    pub span: Span,
}

impl Path {
    pub fn new_span(parts: Vec<Expression>, span: Span) -> Self {
        Self { parts, span }
    }

    pub fn new(parts: Vec<Expression>) -> Self {
        Self::new_span(parts, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        format!("Path({:?})", self.parts)
    }
}

// MARK: Uri
#[derive(Clone, Debug)]
pub struct Uri {
    pub uri: String,
    pub span: Span,
}

impl Uri {
    pub fn new_span(uri: String, span: Span) -> Self {
        Self { uri, span }
    }

    pub fn new(uri: String) -> Self {
        Self::new_span(uri, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        format!("Uri('{}')", self.uri)
    }
}

// ==================== PROPERTY ACCESS ====================
// MARK: PropertyAccess
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

    pub fn debug(&self) -> String {
        format!(
            "PropertyAccess(expr={:?}, path={:?}, default={:?})",
            self.expression, self.attribute_path, self.default
        )
    }
}

// ==================== REMAINING TYPES ====================
// MARK: SearchNixPath
#[derive(Clone, Debug)]
pub struct SearchNixPath {
    pub path: String,
    pub span: Span,
}

impl SearchNixPath {
    pub fn new_span(path: String, span: Span) -> Self {
        Self { path, span }
    }

    pub fn new(path: String) -> Self {
        Self::new_span(path, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        format!("SearchNixPath('{}')", self.path)
    }
}

// MARK: NixString
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

    pub fn debug(&self) -> String {
        format!("String({:?})", self.parts)
    }
}

// MARK: UnaryOperation
#[derive(Clone, Debug)]
pub struct UnaryOperation {
    pub operator: Operator,
    pub operand: Box<Expression>,
    pub span: Span,
}

impl UnaryOperation {
    pub fn new_span(operator: Operator, operand: Expression, span: Span) -> Self {
        Self {
            operator,
            operand: Box::new(operand),
            span,
        }
    }

    pub fn new(operator: Operator, operand: Expression) -> Self {
        Self::new_span(operator, operand, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        format!("UnaryOperation({:?})", self.operator)
    }
}

// MARK: With
#[derive(Clone, Debug)]
pub struct With {
    pub expression: Box<Expression>,
    pub target: Box<Expression>,
    pub span: Span,
}

impl With {
    pub fn new_span(expression: Expression, target: Expression, span: Span) -> Self {
        Self {
            expression: Box::new(expression),
            target: Box::new(target),
            span,
        }
    }

    pub fn new(expression: Expression, target: Expression) -> Self {
        Self::new_span(expression, target, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        format!("With({:?})", self.expression)
    }
}

// ==================== BINDINGS ====================
// MARK: BindingInherit
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
        Self::new_span(from_, attributes, Span::new(Position::new(1, 1), Position::new(1, 1)))
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
    
    pub fn debug(&self) -> String {
        format!("KeyValue({:?})", self.from_)
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    // Core literals
    Integer(Integer),
    Float(Float),
    Identifier(Identifier),
    Error(Error),

    // Operators
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),

    // Control flow
    IfThenElse(IfThenElse),
    Assert(Assert),
    With(With),
    LetIn(LetIn),

    // Collections
    List(List),
    Map(Map),

    // Paths and URIs
    Path(Path),
    Uri(Uri),
    SearchNixPath(SearchNixPath),

    // Strings
    NixString(NixString),
    IndentedString(IndentedString),
    PartRaw(PartRaw),
    PartInterpolation(PartInterpolation),

    // Property access
    PropertyAccess(PropertyAccess),
    HasAttribute(HasAttribute),

    // Functions
    Function(Function),
    FunctionApplication(FunctionApplication),

    // Bindings
    BindingInherit(BindingInherit),
    BindingKeyValue(BindingKeyValue),
}

impl Expression {
    pub fn render(&self) -> String {
        match self {
            Expression::Integer(x) => x.render(),
            Expression::Float(x) => x.render(),
            Expression::Identifier(x) => x.render(),
            // Expression::Error(x) => x.render(),
            // Expression::UnaryOperation(x) => x.render(),
            // Expression::BinaryOperation(x) => x.render(),
            // Expression::IfThenElse(x) => x.render(),
            // Expression::Assert(x) => x.render(),
            // Expression::With(x) => x.render(),
            // Expression::LetIn(x) => x.render(),
            // Expression::List(x) => x.render(),
            // Expression::Map(x) => x.render(),
            // Expression::Path(x) => x.render(),
            // Expression::Uri(x) => x.render(),
            // Expression::SearchNixPath(x) => x.render(),
            // Expression::NixString(x) => x.render(),
            // Expression::IndentedString(x) => x.render(),
            // Expression::PartRaw(x) => x.render(),
            // Expression::PartInterpolation(x) => x.render(),
            // Expression::PropertyAccess(x) => x.render(),
            // Expression::HasAttribute(x) => x.render(),
            // Expression::Function(x) => x.render(),
            // Expression::FunctionApplication(x) => x.render(),
            // Expression::BindingInherit(x) => x.render(),
            // Expression::BindingKeyValue(x) => x.render(),
            _ => String::from(""),
        }
    }
}
