use gorani_span::Span;

pub struct InputObjectTypeExtension {
    pub span: Span,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
    pub input_fields: Vec<crate::InputValueDefinition>,
}
