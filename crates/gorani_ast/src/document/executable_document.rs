use gorani_span::Span;

pub struct ExecutableDocument {
    pub span: Span,
    pub definitions: Vec<crate::ExecutableDefinition>,
}

impl ExecutableDocument {
    pub fn iter(&self) -> impl Iterator<Item = &crate::ExecutableDefinition> {
        self.definitions.iter()
    }
}
