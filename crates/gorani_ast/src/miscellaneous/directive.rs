use gorani_span::Span;

pub struct Directive {
    pub span: Span,
    pub name: crate::Name,
    pub arguments: Vec<crate::Argument>,
}
