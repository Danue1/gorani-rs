use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct VariableNode(pub(crate) SyntaxNode);

impl Node for VariableNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::VARIABLE) {
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

impl Parent<VariableNode> for crate::ValueNode {
    //
}

impl VariableNode {
    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as crate::Parent<crate::NameNode>>::child(self)
    }
}
