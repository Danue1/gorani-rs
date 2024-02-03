use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DescriptionNode(pub(crate) SyntaxNode);

impl Node for DescriptionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DESCRIPTION) {
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

impl Parent<DescriptionNode> for crate::SchemaDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::ScalarTypeDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::ObjectTypeDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::InterfaceTypeDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::UnionTypeDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::EnumTypeDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::InputObjectTypeDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::FieldDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::InputValueDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::EnumValueDefinitionNode {
    //
}

impl Parent<DescriptionNode> for crate::DirectiveDefinitionNode {
    //
}
