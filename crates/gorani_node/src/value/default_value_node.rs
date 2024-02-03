use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DefaultValueNode(pub(crate) SyntaxNode);

impl Node for DefaultValueNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DEFAULT_VALUE) {
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

impl Parent<DefaultValueNode> for crate::InputValueDefinitionNode {
    //
}
