use gorani_span::Span;

pub struct Definition {
    pub span: Span,
    pub kind: DefinitionKind,
}

pub enum DefinitionKind {
    ExecutableDefinition(crate::ExecutableDefinition),
    TypeSystemDefinitionOrExtension(crate::TypeSystemDefinitionOrExtension),
}
