use gorani_span::Span;

pub struct Document {
    pub span: Span,
    pub definitions: Vec<crate::Definition>,
}

impl Document {
    pub fn iter(&self) -> impl Iterator<Item = &crate::Definition> {
        self.definitions.iter()
    }
}
