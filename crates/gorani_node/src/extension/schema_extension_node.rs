use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct SchemaExtensionNode(pub(crate) SyntaxNode);

impl Node for SchemaExtensionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::SCHEMA_EXTENSION) {
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

impl Parent<SchemaExtensionNode> for crate::TypeSystemExtensionNode {
    //
}
