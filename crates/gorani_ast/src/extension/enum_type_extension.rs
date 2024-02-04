use gorani_span::Span;

pub struct EnumTypeExtension {
    pub span: Span,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
    pub enum_values: Vec<crate::EnumValueDefinition>,
}
