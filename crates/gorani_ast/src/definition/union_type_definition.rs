use gorani_span::Span;

pub struct UnionTypeDefinition {
    pub span: Span,
    pub description: Option<crate::Description>,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
    pub union_member_types: Vec<crate::NamedType>,
}
