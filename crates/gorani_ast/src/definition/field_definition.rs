use gorani_span::Span;

pub struct FieldDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub name: crate::Name,
    pub argument_definitions: Vec<crate::InputValueDefinition>,
    pub r#type: crate::Type,
    pub directives: Vec<crate::Directive>,
}
