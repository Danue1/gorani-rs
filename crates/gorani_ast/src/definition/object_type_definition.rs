use gorani_span::Span;

pub struct ObjectTypeDefinition {
    pub span: Span,
    pub name: crate::Name,
    pub interfaces: Vec<crate::NamedType>,
    pub directives: Vec<crate::Directive>,
    pub fields_definition: Vec<crate::FieldDefinition>,
}
