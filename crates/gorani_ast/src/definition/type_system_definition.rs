use gorani_span::Span;

pub struct TypeSystemDefinition {
    pub span: Span,
    pub kind: TypeSystemDefinitionKind,
}

pub enum TypeSystemDefinitionKind {
    SchemaDefinition(crate::SchemaDefinition),
    TypeDefinition(crate::TypeDefinition),
    DirectiveDefinition(crate::DirectiveDefinition),
}
