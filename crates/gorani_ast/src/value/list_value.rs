use gorani_span::Span;

pub struct ListValue {
    pub span: Span,
    pub values: Vec<crate::Value>,
}
