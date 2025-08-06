# Nix Grammar
This document describes all the grammar defined in the AST for the Nix parser

## Grammar Examples
List of all types of grammar

### Core Types
* Identifier: ```variableName```
* Integer: ```42``` 
* Float: ```3.14``` 
* Error: (Runtime error)

### Operators
 * Addition: ```a + b```
 * Subtraction: ```a - b```
 * Multiplication: ```a * b```
 * Division: `a / b`
 * EqualTo: `a == b`
 * NotEqualTo: `a != b`
 * GreaterThan: `a > b`
 * GreaterThanOrEqualTo: `a >= b`
 * LessThan: `a < b`
 * LessThanOrEqualTo: `a <= b`
 * LogicalAnd: `a && b`
 * LogicalOr: `a || b`
 * Not: `!a`
 * Negate: `-a`
 * Concatenation: `a ++ b`
 * Implication: `->`
 * Update: `a := b`

### Functions
 * FunctionHeadDestructuredArgument: `arg ? default`:
 * FunctionHeadDestructured: `{ a, b ? 0 }`:
 * FunctionHeadSimple: `arg`:
 * Function: `arg: body`
 * FunctionApplication: `func arg`

### String Components
 * PartInterpolation: `${expr}`
 * PartRaw: `"text"`

### Control Flow
 * BinaryOperation: `left OP right`
 * Assert: `assert cond; value`
 * HasAttribute: `set ? attr`
 * IfThenElse: `if cond then a else b`
 * LetIn: `let x=1; in x`

### Collections
 * List: `[ a b c ]`
 * Map: (non-recursive) `{ a=1; b=2; }` or (recursive) `rec { a=1; b=a+1; }`

### Paths/URIs
 * Path: `./path/to/file`
 * Uri: `"https://example.com"`
 * SearchNixPath: `<nixpkgs>`

### Strings
 * NixString: `"text ${expr}"`
 * IndentedString: `''multiline ${expr}''`

### Property Access
 * PropertyAccess: `obj.attr`
 * UnaryOperation: `OP operand` (e.g., `!a`)

### Scoping
 * With: `with pkgs; [ hello ]`

### Bindings
 * BindingInherit: `inherit attr;` or `inherit (set) attr`;
 * BindingKeyValue: `name = value`;

### Enums
 * FunctionHead: (Parent of simple/destructured heads)
 * Expression: (Parent of all expression types)

## List
 * Position
 * Span
 * Identifier
 * Error
 * Float
 * Integer
 * Addition
 * Subtraction
 * Multiplication
 * Division
 * EqualTo
 * NotEqualTo
 * GreaterThan
 * GreaterThanOrEqualTo
 * LessThan
 * LessThanOrEqualTo
 * LogicalAnd
 * LogicalOr
 * Not
 * Negate
 * Concatenation
 * Implication
 * Update
 * FunctionHeadDestructuredArgument
 * FunctionHeadDestructured
 * FunctionHeadSimple
 * Function
 * FunctionApplication
 * PartInterpolation
 * PartRaw
 * BinaryOperation
 * Assert
 * HasAttribute
 * IndentedString
 * IfThenElse
 * LetIn
 * List
 * Map
 * Path
 * Uri
 * PropertyAccess
 * SearchNixPath
 * NixString
 * UnaryOperation
 * With
 * BindingInherit
 * BindingKeyValue

## Structs
```rust
pub struct Position {
    pub line: i64,
    pub column: i64,
}

pub struct Span {
    pub start: Position,
    pub end: Position,
}

pub struct Identifier {
    pub id: String,
    pub span: Span,
}

pub struct Error {
    pub message: String,
    pub span: Span,
}

pub struct Float {
    pub value: String,
    pub span: Span,
}

pub struct Integer {
    pub value: String,
    pub span: Span,
}

pub struct Addition;
pub struct Subtraction;
pub struct Multiplication;
pub struct Division;
pub struct EqualTo ;
pub struct NotEqualTo ;
pub struct GreaterThan;
pub struct GreaterThanOrEqualTo ;
pub struct LessThan;
pub struct LessThanOrEqualTo ;
pub struct LogicalAnd ;
pub struct LogicalOr ;
pub struct Not;
pub struct Negate;
pub struct Concatenation ;
pub struct Implication ;
pub struct Update ;

pub struct FunctionHeadDestructuredArgument {
    pub identifier: String,
    pub default: Option<Expression>,
}

pub struct FunctionHeadDestructured {
    pub ellipsis: bool,
    pub identifier: Identifier,
    pub arguments: Vec<FunctionHeadDestructuredArgument>,
    pub span: Span,
}

pub struct FunctionHeadSimple {
    pub identifier: Identifier,
    pub span: Span,
}

pub enum FunctionHead {
    FunctionHeadSimple(FunctionHeadSimple),
    FunctionHeadDestructured(FunctionHeadDestructured),
}

pub struct Function {
    pub head: Box<Expression>,
    pub body: Box<Expression>,
    pub span: Span,
}

pub struct FunctionApplication {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

pub struct PartInterpolation {
    pub expression: Box<Expression>,
    pub span: Span,
}

pub struct PartRaw {
    pub content: String,
    pub span: Span,
}

pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
    pub span: Span,
}

pub struct Assert {
    pub expression: Box<Expression>,
    pub target: Box<Expression>,
    pub span: Span,
}

pub struct HasAttribute {
    pub expression: Box<Expression>,
    pub attribute_path: Vec<Expression>,
    pub span: Span,
}

pub struct IndentedString {
    pub parts: Vec<Expression>,
    pub span: Span,
}

pub struct IfThenElse {
    pub predicate: Box<Expression>,
    pub then: Box<Expression>,
    pub else_: Box<Expression>,
    pub span: Span,
}

pub struct LetIn {
    pub bindings: Vec<Expression>,
    pub target: Box<Expression>,
    pub span: Span,
}

pub struct List {
    pub elements: Vec<Expression>,
    pub span: Span,
}

pub struct Map {
    pub recursive: bool,
    pub bindings: Vec<Expression>,
    pub span: Span,
}

pub struct Path {
    pub parts: Vec<Expression>,
    pub span: Span,
}

pub struct Uri {
    pub uri: String,
    pub span: Span,
}

pub struct PropertyAccess {
    pub expression: Box<Expression>,
    pub attribute_path: Vec<Expression>,
    pub default: Option<Box<Expression>>,
    pub span: Span,
}

pub struct SearchNixPath {
    pub path: String,
    pub span: Span,
}

pub struct NixString {
    pub parts: Vec<Expression>,
    pub span: Span,
}

pub struct UnaryOperation {
    pub operator: Operator,
    pub operand: Box<Expression>,
    pub span: Span,
}

pub struct With {
    pub expression: Box<Expression>,
    pub target: Box<Expression>,
    pub span: Span,
}

pub struct BindingInherit {
    pub from_: Option<Box<Expression>>,
    pub attributes: Box<Expression>,
    pub span: Span,
}

// MARK: BindingKeyValue
pub struct BindingKeyValue {
    pub from_: Box<Expression>,
    pub to: Box<Expression>,
}

pub enum Expression {
    Integer(Integer),
    Float(Float),
    Identifier(Identifier),
    Error(Error),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    IfThenElse(IfThenElse),
    Assert(Assert),
    With(With),
    LetIn(LetIn),
    List(List),
    Map(Map),
    Path(Path),
    Uri(Uri),
    SearchNixPath(SearchNixPath),
    NixString(NixString),
    IndentedString(IndentedString),
    PartRaw(PartRaw),
    PartInterpolation(PartInterpolation),
    PropertyAccess(PropertyAccess),
    HasAttribute(HasAttribute),
    Function(Function),
    FunctionApplication(FunctionApplication),
    BindingInherit(BindingInherit),
    BindingKeyValue(BindingKeyValue),
}
```