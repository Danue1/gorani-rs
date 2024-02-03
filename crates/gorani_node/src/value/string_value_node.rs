use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct StringValueNode(pub(crate) SyntaxNode);

impl Node for StringValueNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::STRING_VALUE) {
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

impl Parent<StringValueNode> for crate::ValueNode {
    //
}

impl StringValueNode {
    pub fn value(&self) -> Option<String> {
        Some(self.0.first_child()?.to_string())
    }
}
