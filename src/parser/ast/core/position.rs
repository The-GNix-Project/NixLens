use pest::Span as PestSpan;

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

    pub fn from_pest_span(span: PestSpan) -> Self {
        let (start_line, start_col) = span.start_pos().line_col();
        let (end_line, end_col) = span.end_pos().line_col();
        Span::new(
            Position::new(start_line as i64, start_col as i64),
            Position::new(end_line as i64, end_col as i64),
        )
    }
}
