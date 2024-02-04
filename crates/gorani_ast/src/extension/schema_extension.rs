use gorani_span::Span;

pub struct SchemaExtension {
    pub span: Span,
    pub directives: Vec<crate::Directive>,
    pub root_operation_type_definitions: Vec<crate::RootOperationTypeDefinition>,
}
