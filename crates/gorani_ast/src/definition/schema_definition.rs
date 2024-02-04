use gorani_span::Span;

pub struct SchemaDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub directives: Vec<crate::Directive>,
    pub root_operation_type_definitions: Vec<crate::RootOperationTypeDefinition>,
}
