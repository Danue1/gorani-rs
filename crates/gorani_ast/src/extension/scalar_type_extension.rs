use gorani_span::Span;

pub struct ScalarTypeExtension {
    pub span: Span,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
}
