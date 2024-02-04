use gorani_span::Span;

pub struct TypeSystemExtension {
    pub span: Span,
    pub kind: TypeSystemExtensionKind,
}

pub enum TypeSystemExtensionKind {
    SchemaExtension(crate::SchemaExtension),
    TypeExtension(crate::TypeExtension),
}
