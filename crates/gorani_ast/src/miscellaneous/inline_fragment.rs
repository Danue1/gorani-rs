use gorani_span::Span;

pub struct InlineFragment {
    pub span: Span,
    pub type_condition: Option<crate::NamedType>,
    pub directives: Vec<crate::Directive>,
    pub selections: Vec<crate::Selection>,
}
