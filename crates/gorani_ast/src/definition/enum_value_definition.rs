use gorani_span::Span;

pub struct EnumValueDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
}
