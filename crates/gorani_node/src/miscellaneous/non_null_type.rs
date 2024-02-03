use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct NonNullTypeNode(pub(crate) SyntaxNode);

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
