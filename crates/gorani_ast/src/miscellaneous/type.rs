use gorani_span::Span;

pub struct Type {
    pub span: Span,
    pub kind: TypeKind,
}

pub enum TypeKind {
    NamedType(crate::NamedType),
    ListType(crate::ListType),
    NonNullType(crate::NonNullType),
}
