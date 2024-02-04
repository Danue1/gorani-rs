use gorani_span::Span;

pub struct NonNullType {
    pub span: Span,
    pub kind: NonNullTypeKind,
}

pub enum NonNullTypeKind {
    NamedType(crate::NamedType),
    ListType(crate::ListType),
}
