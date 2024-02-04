use gorani_span::Span;

pub struct TypeSystemDefinitionOrExtension {
    pub span: Span,
    pub kind: TypeSystemDefinitionOrExtensionKind,
}

pub enum TypeSystemDefinitionOrExtensionKind {
    TypeSystemDefinition(crate::TypeSystemDefinition),
    TypeSystemExtension(crate::TypeSystemExtension),
}
