use gorani_span::Span;

pub struct TypeSystemDocument {
    pub span: Span,
    pub definitions: Vec<crate::TypeSystemDefinition>,
}

impl TypeSystemDocument {
    pub fn iter(&self) -> impl Iterator<Item = &crate::TypeSystemDefinition> {
        self.definitions.iter()
    }
}
