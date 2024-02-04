use gorani_span::Span;

pub struct InterfaceTypeExtension {
    pub span: Span,
    pub name: crate::Name,
    pub interfaces: Vec<crate::NamedType>,
    pub directives: Vec<crate::Directive>,
    pub field_definitions: Vec<crate::FieldDefinition>,
}
