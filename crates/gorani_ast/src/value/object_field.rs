use gorani_span::Span;

pub struct ObjectField {
    pub span: Span,
    pub name: crate::Name,
    pub value: crate::Value,
}
