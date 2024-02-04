use gorani_span::Span;

pub struct DirectiveDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub name: crate::Name,
    pub argument_definitions: Vec<crate::InputValueDefinition>,
    pub repeatable: bool,
    pub locations: Vec<crate::DirectiveLocation>,
}
