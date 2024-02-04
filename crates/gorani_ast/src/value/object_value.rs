use gorani_span::Span;

pub struct ObjectValue {
    pub span: Span,
    pub object_fields: Vec<crate::ObjectField>,
}
