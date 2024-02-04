use gorani_span::Span;

pub struct EnumTypeDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
    pub enum_value_definitions: Vec<crate::EnumValueDefinition>,
}
