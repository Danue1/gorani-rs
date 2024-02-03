use crate::{ListTypeNode, Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeNode(pub(crate) SyntaxNode);

impl Node for TypeNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE) {
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

impl Parent<crate::TypeNode> for ListTypeNode {
    //
}

impl Parent<crate::TypeNode> for crate::FieldDefinitionNode {
    //
}

impl Parent<crate::TypeNode> for crate::InputValueDefinitionNode {
    //
}

impl TypeNode {
    pub fn named_type(&self) -> Option<crate::NamedTypeNode> {
        <Self as Parent<crate::NamedTypeNode>>::child(self)
    }

    pub fn list_type(&self) -> Option<crate::ListTypeNode> {
        <Self as Parent<crate::ListTypeNode>>::child(self)
    }

    pub fn non_null_type(&self) -> Option<crate::NonNullTypeNode> {
        <Self as Parent<crate::NonNullTypeNode>>::child(self)
    }
}
