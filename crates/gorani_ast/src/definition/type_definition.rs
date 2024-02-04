use gorani_span::Span;

pub struct TypeDefinition {
    pub span: Span,
    pub kind: TypeDefinitionKind,
}

pub enum TypeDefinitionKind {
    ScalarTypeDefinition(crate::ScalarTypeDefinition),
    ObjectTypeDefinition(crate::ObjectTypeDefinition),
    InterfaceTypeDefinition(crate::InterfaceTypeDefinition),
    UnionTypeDefinition(crate::UnionTypeDefinition),
    EnumTypeDefinition(crate::EnumTypeDefinition),
    InputObjectTypeDefinition(crate::InputObjectTypeDefinition),
}
