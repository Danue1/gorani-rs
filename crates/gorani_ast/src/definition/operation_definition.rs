use gorani_span::Span;

pub struct OperationDefinition {
    pub span: Span,
    pub operation_type: Option<crate::OperationType>,
    pub name: Option<crate::Name>,
    pub variable_definitions: Vec<crate::VariableDefinition>,
    pub directives: Vec<crate::Directive>,
    pub selection_set: crate::SelectionSet,
}
