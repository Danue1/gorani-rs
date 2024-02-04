use gorani_span::Span;

pub struct ScalarTypeDefinition {
    pub span: Span,
    pub name: crate::Name,
    pub directives: Vec<crate::Directive>,
}
