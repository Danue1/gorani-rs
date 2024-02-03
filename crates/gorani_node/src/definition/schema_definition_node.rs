use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct SchemaDefinitionNode(pub(crate) SyntaxNode);

impl Node for SchemaDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::SCHEMA_DEFINITION) {
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

impl Parent<SchemaDefinitionNode> for crate::TypeSystemDefinitionNode {
    //
}

impl SchemaDefinitionNode {
    pub fn description(&self) -> Option<crate::DescriptionNode> {
        <Self as Parent<crate::DescriptionNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn root_operation_type_definition(
        &self,
    ) -> impl Iterator<Item = crate::RootOperationTypeDefinitionNode> + '_ {
        <Self as Parent<crate::RootOperationTypeDefinitionNode>>::children(self)
    }
}
