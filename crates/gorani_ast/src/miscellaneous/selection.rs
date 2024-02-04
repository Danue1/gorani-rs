use gorani_span::Span;

pub struct Selection {
    pub span: Span,
    pub kind: SelectionKind,
}

pub enum SelectionKind {
    Field(crate::Field),
    FragmentSpread(crate::FragmentSpread),
    InlineFragment(crate::InlineFragment),
}
