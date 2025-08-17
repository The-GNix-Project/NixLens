use std::fmt;

#[derive(Debug)]
pub enum RenderError {
    InvalidNode(String),
    UnexpectedError(String),
    InvalidAst(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::InvalidNode(msg) => write!(f, "Invalid node: {}", msg),
            RenderError::UnexpectedError(msg) => write!(f, "Unexpected render error: {}", msg),
            RenderError::InvalidAst(msg) => write!(f, "Invalid AST: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error: {}", self.message)
    }
}

impl std::error::Error for SyntaxError {}
