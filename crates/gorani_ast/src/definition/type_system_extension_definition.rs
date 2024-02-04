use gorani_span::Span;

pub struct TypeSystemExtensionDefinition {
    pub span: Span,
    pub kind: TypeSystemExtensionDefinitionKind,
}

pub enum TypeSystemExtensionDefinitionKind {
    TypeSystemDefinition(crate::TypeSystemDefinition),
    TypeSystemExtension(crate::TypeSystemExtension),
}
