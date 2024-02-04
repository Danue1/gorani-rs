use gorani_span::Span;

pub struct SelectionSet {
    pub span: Span,
    pub selections: Vec<crate::Selection>,
}
