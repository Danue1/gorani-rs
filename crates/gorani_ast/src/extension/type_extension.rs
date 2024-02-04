use gorani_span::Span;

pub struct TypeExtension {
    pub span: Span,
    pub kind: TypeExtensionKind,
}

pub enum TypeExtensionKind {
    ScalarTypeExtension(crate::ScalarTypeExtension),
    ObjectTypeExtension(crate::ObjectTypeExtension),
    InterfaceTypeExtension(crate::InterfaceTypeExtension),
    UnionTypeExtension(crate::UnionTypeExtension),
    EnumTypeExtension(crate::EnumTypeExtension),
    InputObjectTypeExtension(crate::InputObjectTypeExtension),
}
