#[derive(Clone, Debug)]
pub struct Position {
    pub line: i64,
    pub column: i64,
}

impl Position {
    pub fn new(line: i64, column: i64) -> Self {
        Self { line, column }
    }
}

#[derive(Clone, Debug)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}