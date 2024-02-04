use gorani_span::Span;

pub struct UnionTypeExtension {
    pub span: Span,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
    pub union_member_types: Vec<crate::NamedType>,
}
