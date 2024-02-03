use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeSystemDirectiveLocationNode(pub(crate) SyntaxNode);

#[allow(non_camel_case_types)]
pub enum TypeSystemDirectiveLocation {
    SCHEMA,
    SCALAR,
    OBJECT,
    FIELD_DEFINITION,
    ARGUMENT_DEFINITION,
    INTERFACE,
    UNION,
    ENUM,
    ENUM_VALUE,
    INPUT_OBJECT,
    INPUT_FIELD_DEFINITION,
}

impl Node for TypeSystemDirectiveLocationNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_SYSTEM_DIRECTIVE_LOCATION) {
            Some(Self(node))
        } else {
            None
        }
    }

    #[inline]
    fn node(&self) -> SyntaxNode {
        self.0.clone()
    }
}

impl Parent<TypeSystemDirectiveLocationNode> for crate::DirectiveLocationNode {
    //
}

impl TypeSystemDirectiveLocationNode {
    pub fn kind(&self) -> Option<TypeSystemDirectiveLocation> {
        match self.0.first_child() {
            Some(child) => match child.kind() {
                SyntaxKind::LOCATION_SCHEMA => Some(TypeSystemDirectiveLocation::SCHEMA),
                SyntaxKind::LOCATION_SCALAR => Some(TypeSystemDirectiveLocation::SCALAR),
                SyntaxKind::LOCATION_OBJECT => Some(TypeSystemDirectiveLocation::OBJECT),
                SyntaxKind::LOCATION_FIELD_DEFINITION => {
                    Some(TypeSystemDirectiveLocation::FIELD_DEFINITION)
                }
                SyntaxKind::LOCATION_ARGUMENT_DEFINITION => {
                    Some(TypeSystemDirectiveLocation::ARGUMENT_DEFINITION)
                }
                SyntaxKind::LOCATION_INTERFACE => Some(TypeSystemDirectiveLocation::INTERFACE),
                SyntaxKind::LOCATION_UNION => Some(TypeSystemDirectiveLocation::UNION),
                SyntaxKind::LOCATION_ENUM => Some(TypeSystemDirectiveLocation::ENUM),
                SyntaxKind::LOCATION_ENUM_VALUE => Some(TypeSystemDirectiveLocation::ENUM_VALUE),
                SyntaxKind::LOCATION_INPUT_OBJECT => {
                    Some(TypeSystemDirectiveLocation::INPUT_OBJECT)
                }
                SyntaxKind::LOCATION_INPUT_FIELD_DEFINITION => {
                    Some(TypeSystemDirectiveLocation::INPUT_FIELD_DEFINITION)
                }
                _ => None,
            },
            _ => None,
        }
    }
}
