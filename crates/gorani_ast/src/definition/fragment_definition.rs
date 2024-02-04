use gorani_span::Span;

pub struct FragmentDefinition {
    pub span: Span,
    pub name: crate::Name,
    pub type_condition: crate::NamedType,
    pub directives: Vec<crate::Directive>,
    pub selection_set: crate::SelectionSet,
}
