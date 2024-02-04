use gorani_span::Span;

pub struct InputObjectTypeDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
    pub input_value_definitions: Vec<crate::InputValueDefinition>,
}
