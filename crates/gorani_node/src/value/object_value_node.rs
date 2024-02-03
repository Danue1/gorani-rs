use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ObjectValueNode(pub(crate) SyntaxNode);

impl Node for ObjectValueNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::OBJECT_VALUE) {
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

impl Parent<ObjectValueNode> for crate::ValueNode {
    //
}

impl ObjectValueNode {
    pub fn left_brace(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_LEFT_BRACE))
    }

    pub fn object_fields(&self) -> impl Iterator<Item = crate::ObjectFieldNode> + '_ {
        <Self as Parent<crate::ObjectFieldNode>>::children(self)
    }

    pub fn right_brace(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_RIGHT_BRACE))
    }
}
