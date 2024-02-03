use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ListValueNode(pub(crate) SyntaxNode);

impl Node for ListValueNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::LIST_VALUE) {
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

impl Parent<ListValueNode> for crate::ValueNode {
    //
}

impl ListValueNode {
    pub fn values(&self) -> impl Iterator<Item = crate::ValueNode> + '_ {
        <Self as Parent<crate::ValueNode>>::children(self)
    }
}
