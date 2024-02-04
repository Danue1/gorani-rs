use gorani_span::Span;

pub struct InputValueDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub name: crate::Name,
    pub r#type: crate::Type,
    pub default_value: Option<crate::Value>,
    pub directives: Vec<crate::Directive>,
}
