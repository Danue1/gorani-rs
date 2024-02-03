use crate::Node;
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ExecutableDocumentNode(pub(crate) SyntaxNode);

impl Node for ExecutableDocumentNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::EXECUTABLE_DOCUMENT) {
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

impl ExecutableDocumentNode {
    pub fn executable_definitions(
        &self,
    ) -> impl Iterator<Item = crate::ExecutableDefinitionNode> + '_ {
        <Self as crate::Parent<crate::ExecutableDefinitionNode>>::children(self)
    }
}
