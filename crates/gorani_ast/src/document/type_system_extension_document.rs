use gorani_span::Span;

pub struct TypeSystemExtensionDocument {
    pub span: Span,
    pub definitions: Vec<crate::TypeSystemExtensionDefinition>,
}

impl TypeSystemExtensionDocument {
    pub fn iter(&self) -> impl Iterator<Item = &crate::TypeSystemExtensionDefinition> {
        self.definitions.iter()
    }
}
