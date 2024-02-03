use crate::Node;
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeSystemExtensionDocumentNode(pub(crate) SyntaxNode);

impl Node for TypeSystemExtensionDocumentNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_SYSTEM_EXTENSION_DOCUMENT) {
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

impl TypeSystemExtensionDocumentNode {
    pub fn type_system_definition_or_extensions(
        &self,
    ) -> impl Iterator<Item = crate::TypeSystemDefinitionOrExtensionNode> + '_ {
        <Self as crate::Parent<crate::TypeSystemDefinitionOrExtensionNode>>::children(self)
    }
}
