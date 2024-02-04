use gorani_span::Span;

pub struct Argument {
    pub span: Span,
    pub name: crate::Name,
    pub value: crate::Value,
}
