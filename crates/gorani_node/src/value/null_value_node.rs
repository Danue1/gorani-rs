use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct NullValueNode(pub(crate) SyntaxNode);

impl Node for NullValueNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::NULL_VALUE) {
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

impl Parent<NullValueNode> for crate::ValueNode {
    //
}

impl NullValueNode {
    pub fn value(&self) -> bool {
        match self.0.first_child() {
            Some(child) => child.kind() == SyntaxKind::SYMBOL_NULL,
            None => false,
        }
    }
}
