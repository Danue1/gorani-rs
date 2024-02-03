use crate::Node;
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DocumentNode(pub(crate) SyntaxNode);

impl Node for DocumentNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DOCUMENT) {
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

impl DocumentNode {
    pub fn definitions(&self) -> impl Iterator<Item = crate::DefinitionNode> + '_ {
        <Self as crate::Parent<crate::DefinitionNode>>::children(self)
    }
}
