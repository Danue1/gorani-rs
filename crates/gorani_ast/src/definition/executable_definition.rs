use gorani_span::Span;

pub struct ExecutableDefinition {
    pub span: Span,
    pub kind: ExecutableDefinitionKind,
}

pub enum ExecutableDefinitionKind {
    OperationDefinition(crate::OperationDefinition),
    FragmentDefinition(crate::FragmentDefinition),
}
