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

impl SchemaExtensionNode {
    pub fn extend(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_EXTEND))
    }

    pub fn schema(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_SCHEMA))
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn left_brace(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_LEFT_BRACE))
    }

    pub fn root_operation_type_definitions(
        &self,
    ) -> impl Iterator<Item = crate::RootOperationTypeDefinitionNode> + '_ {
        <Self as crate::Parent<crate::RootOperationTypeDefinitionNode>>::children(self)
    }

    pub fn right_brace(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_RIGHT_BRACE))
    }
}
