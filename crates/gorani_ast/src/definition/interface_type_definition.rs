use gorani_span::Span;

pub struct InterfaceTypeDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub name: crate::Name,
    pub interfaces: Vec<crate::NamedType>,
    pub directives: Vec<crate::Directive>,
    pub field_definitions: Vec<crate::FieldDefinition>,
}
