use super::position::{Position, Span};

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
        Self::new_span(value, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        format!("Integer('{}')", self.value)
    }

    pub fn render(&self) -> String {
        format!("{}", self.value)
    }
}

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

#[derive(Clone, Debug)]
pub struct LiteralString {
    pub value: String,
    pub span: Span,
}

impl LiteralString {
    pub fn new_span(value: String, span: Span) -> Self {
        Self { value, span }
    }

    pub fn new(value: String) -> Self {
        Self::new_span(value, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        format!("LiteralString('{}')", self.value)
    }

    pub fn render(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    pub parts: Vec<String>,
    pub span: Span,
}

impl Path {
    pub fn new_span(parts: Vec<String>, span: Span) -> Self {
        Self { parts, span }
    }

    pub fn new(parts: Vec<String>) -> Self {
        Self::new_span(parts, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn render(&self) -> String {
        self.parts.join("/")
    }

    pub fn debug(&self) -> String {
        format!("Path({:?})", self.parts)
    }
}

#[derive(Clone, Debug)]
pub struct Boolean {
    pub value: bool,
    pub span: Span
}

impl Boolean {
    pub fn new_span(value: bool, span: Span) -> Self {
        Self { value, span }
    }

    pub fn new(value: bool) -> Self {
        Self::new_span(value, Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        format!("Integer('{}')", self.value)
    }

    pub fn render(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Clone, Debug)]
pub struct Null {
    pub span: Span
}

impl Null {
    pub fn new_span(span: Span) -> Self {
        Self { span }
    }

    pub fn new() -> Self {
        Self::new_span(Span::new(Position::new(1, 1), Position::new(1, 1)))
    }

    pub fn debug(&self) -> String {
        "Null".to_string()
    }

    pub fn render(&self) -> String {
        "null".to_string()
    }
}

