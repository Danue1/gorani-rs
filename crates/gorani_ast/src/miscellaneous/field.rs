use gorani_span::Span;

pub struct Field {
    pub span: Span,
    pub alias: Option<crate::Name>,
    pub name: crate::Name,
    pub arguments: Vec<crate::Argument>,
    pub directives: Vec<crate::Directive>,
    pub selection_set: Option<crate::SelectionSet>,
}
