use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct EnumTypeDefinitionNode(pub(crate) SyntaxNode);

impl Node for EnumTypeDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::ENUM_TYPE_DEFINITION) {
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

impl Parent<EnumTypeDefinitionNode> for crate::TypeDefinitionNode {
    //
}

impl EnumTypeDefinitionNode {
    pub fn description(&self) -> Option<crate::DescriptionNode> {
        <Self as Parent<crate::DescriptionNode>>::child(self)
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn enum_values_definition(&self) -> Option<crate::EnumValuesDefinitionNode> {
        <Self as Parent<crate::EnumValuesDefinitionNode>>::child(self)
    }
}
