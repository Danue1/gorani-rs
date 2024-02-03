use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ObjectFieldNode(pub(crate) SyntaxNode);

impl Node for ObjectFieldNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::OBJECT_FIELD) {
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

impl Parent<ObjectFieldNode> for crate::ObjectValueNode {
    //
}

impl ObjectFieldNode {
    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }

    pub fn value(&self) -> Option<crate::ValueNode> {
        <Self as Parent<crate::ValueNode>>::child(self)
    }
}
