use gorani_span::Span;

pub struct DirectiveLocation {
    pub span: Span,
    pub kind: DirectiveLocationKind,
}

pub enum DirectiveLocationKind {
    ExecutableDirectiveLocation(crate::ExecutableDirectiveLocation),
    TypeSystemDirectiveLocation(crate::TypeSystemDirectiveLocation),
}
