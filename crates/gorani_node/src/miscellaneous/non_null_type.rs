use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct NonNullTypeNode(pub(crate) SyntaxNode);

pub enum NonNullTypeKindNode {
    NamedType(crate::NamedTypeNode),
    ListType(crate::ListTypeNode),
}

impl Node for NonNullTypeNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::NON_NULL_TYPE) {
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

impl Parent<NonNullTypeNode> for crate::NamedTypeNode {
    //
}

impl Parent<NonNullTypeNode> for crate::TypeNode {
    //
}

impl NonNullTypeKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if crate::NamedTypeNode::cast(node.clone()).is_some() {
            Some(Self::NamedType(crate::NamedTypeNode::cast(node)?))
        } else if crate::ListTypeNode::cast(node.clone()).is_some() {
            Some(Self::ListType(crate::ListTypeNode::cast(node)?))
        } else {
            None
        }
    }
}

impl NonNullTypeNode {
    pub fn kind(&self) -> Option<NonNullTypeKindNode> {
        self.0.children().find_map(NonNullTypeKindNode::cast)
    }

    pub fn named_type(&self) -> Option<crate::NamedTypeNode> {
        <Self as Parent<crate::NamedTypeNode>>::child(self)
    }

    pub fn list_type(&self) -> Option<crate::ListTypeNode> {
        <Self as Parent<crate::ListTypeNode>>::child(self)
    }

    pub fn exclamation(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_EXCLAMATION))
    }
}
