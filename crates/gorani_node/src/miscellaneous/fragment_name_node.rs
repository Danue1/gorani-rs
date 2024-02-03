use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct FragmentNameNode(pub(crate) SyntaxNode);

impl Node for FragmentNameNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::FRAGMENT_NAME) {
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

impl Parent<FragmentNameNode> for crate::FragmentDefinitionNode {
    //
}
