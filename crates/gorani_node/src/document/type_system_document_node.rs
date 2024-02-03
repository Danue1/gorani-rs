use crate::Node;
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeSystemDocumentNode(pub(crate) SyntaxNode);

impl Node for TypeSystemDocumentNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_SYSTEM_DOCUMENT) {
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

impl TypeSystemDocumentNode {
    pub fn type_system_definitions(
        &self,
    ) -> impl Iterator<Item = crate::TypeSystemDefinitionNode> + '_ {
        <Self as crate::Parent<crate::TypeSystemDefinitionNode>>::children(self)
    }
}
