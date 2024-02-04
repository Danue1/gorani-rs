#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: Location,
    end: Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Span {
    #[inline]
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn start(&self) -> Location {
        self.start
    }

    #[inline]
    pub fn end(&self) -> Location {
        self.end
    }

    #[inline]
    pub fn with_start(self, start: Location) -> Self {
        Self { start, ..self }
    }

    #[inline]
    pub fn with_end(self, end: Location) -> Self {
        Self { end, ..self }
    }
}

impl Location {
    #[inline]
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }
}
