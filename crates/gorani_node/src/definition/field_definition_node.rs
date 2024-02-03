use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct FieldDefinitionNode(pub(crate) SyntaxNode);

impl Node for FieldDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::FIELD_DEFINITION) {
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

impl Parent<FieldDefinitionNode> for crate::FieldsDefinitionNode {
    //
}

impl FieldDefinitionNode {
    pub fn description(&self) -> Option<crate::DescriptionNode> {
        <Self as Parent<crate::DescriptionNode>>::child(self)
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }

    pub fn arguments_definition(&self) -> Option<crate::ArgumentsDefinitionNode> {
        <Self as Parent<crate::ArgumentsDefinitionNode>>::child(self)
    }

    pub fn r#type(&self) -> Option<crate::TypeNode> {
        <Self as Parent<crate::TypeNode>>::child(self)
    }
}
