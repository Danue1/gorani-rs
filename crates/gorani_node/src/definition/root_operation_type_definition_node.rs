use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct RootOperationTypeDefinitionNode(pub(crate) SyntaxNode);

impl Node for RootOperationTypeDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::ROOT_OPERATION_TYPE_DEFINITION) {
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

impl Parent<RootOperationTypeDefinitionNode> for crate::SchemaDefinitionNode {
    //
}

impl Parent<RootOperationTypeDefinitionNode> for crate::SchemaExtensionNode {
    //
}

impl RootOperationTypeDefinitionNode {
    pub fn operation_type(&self) -> Option<crate::OperationTypeNode> {
        <Self as Parent<crate::OperationTypeNode>>::child(self)
    }

    pub fn colon(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_COLON))
    }

    pub fn named_type(&self) -> Option<crate::NamedTypeNode> {
        <Self as Parent<crate::NamedTypeNode>>::child(self)
    }
}
