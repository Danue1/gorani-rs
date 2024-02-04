use gorani_span::Span;

pub struct FragmentSpread {
    pub span: Span,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
}
