use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ListTypeNode(pub(crate) SyntaxNode);

impl Node for ListTypeNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::LIST_TYPE) {
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

impl Parent<ListTypeNode> for crate::TypeNode {
    //
}

impl Parent<ListTypeNode> for crate::NonNullTypeNode {
    //
}

impl ListTypeNode {
    pub fn left_bracket(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_LEFT_BRACKET))
    }

    pub fn r#type(&self) -> Option<crate::TypeNode> {
        <Self as Parent<crate::TypeNode>>::child(self)
    }

    pub fn right_bracket(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_RIGHT_BRACKET))
    }
}
