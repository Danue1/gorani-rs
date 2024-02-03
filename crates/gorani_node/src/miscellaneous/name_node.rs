use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct NameNode(pub(crate) SyntaxNode);

impl Node for NameNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::NAME) {
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

impl Parent<NameNode> for crate::OperationDefinitionNode {
    //
}

impl Parent<NameNode> for crate::FieldNode {
    //
}

impl Parent<NameNode> for crate::ArgumentNode {
    //
}

impl Parent<NameNode> for crate::VariableNode {
    //
}

impl Parent<NameNode> for crate::ObjectFieldNode {
    //
}

impl Parent<NameNode> for crate::DirectiveNode {
    //
}

impl Parent<NameNode> for crate::FragmentSpreadNode {
    //
}

impl Parent<NameNode> for crate::ScalarTypeDefinitionNode {
    //
}

impl Parent<NameNode> for crate::ObjectTypeDefinitionNode {
    //
}

impl Parent<NameNode> for crate::InterfaceTypeDefinitionNode {
    //
}

impl Parent<NameNode> for crate::UnionTypeDefinitionNode {
    //
}

impl Parent<NameNode> for crate::EnumTypeDefinitionNode {
    //
}

impl Parent<NameNode> for crate::InputObjectTypeDefinitionNode {
    //
}

impl Parent<NameNode> for crate::FieldDefinitionNode {
    //
}

impl Parent<NameNode> for crate::InputValueDefinitionNode {
    //
}

impl Parent<NameNode> for crate::EnumValueDefinitionNode {
    //
}

impl Parent<NameNode> for crate::DirectiveDefinitionNode {
    //
}

impl Parent<NameNode> for crate::ScalarTypeExtensionNode {
    //
}

impl Parent<NameNode> for crate::ObjectTypeExtensionNode {
    //
}

impl Parent<NameNode> for crate::InterfaceTypeExtensionNode {
    //
}

impl Parent<NameNode> for crate::UnionTypeExtensionNode {
    //
}

impl Parent<NameNode> for crate::EnumTypeExtensionNode {
    //
}

impl Parent<NameNode> for crate::InputObjectTypeExtensionNode {
    //
}
