use gorani_span::Span;

pub struct ListType {
    pub span: Span,
    pub r#type: Box<crate::Type>,
}
