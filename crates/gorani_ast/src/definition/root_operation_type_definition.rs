use gorani_span::Span;

pub struct RootOperationTypeDefinition {
    pub span: Span,
    pub operation: crate::OperationType,
    pub r#type: crate::NamedType,
}
