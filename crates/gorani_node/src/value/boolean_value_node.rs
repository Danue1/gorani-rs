use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct BooleanValueNode(pub(crate) SyntaxNode);

impl Node for BooleanValueNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::BOOLEAN_VALUE) {
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

impl Parent<BooleanValueNode> for crate::ValueNode {
    //
}

impl BooleanValueNode {
    pub fn value(&self) -> Option<bool> {
        match self.0.first_child()?.kind() {
            SyntaxKind::SYMBOL_TRUE => Some(true),
            SyntaxKind::SYMBOL_FALSE => Some(false),
            _ => None,
        }
    }
}
