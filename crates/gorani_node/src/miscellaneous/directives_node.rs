use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DirectivesNode(pub(crate) SyntaxNode);

impl Node for DirectivesNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DIRECTIVES) {
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

impl Parent<DirectivesNode> for crate::OperationDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::FieldNode {
    //
}

impl Parent<DirectivesNode> for crate::FragmentSpreadNode {
    //
}

impl Parent<DirectivesNode> for crate::InlineFragmentNode {
    //
}

impl Parent<DirectivesNode> for crate::FragmentDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::SchemaDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::ScalarTypeDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::ObjectTypeDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::InterfaceTypeDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::UnionTypeDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::EnumTypeDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::InputObjectTypeDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::InputValueDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::EnumValueDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::ScalarTypeExtensionNode {
    //
}

impl Parent<DirectivesNode> for crate::ObjectTypeExtensionNode {
    //
}

impl Parent<DirectivesNode> for crate::InterfaceTypeExtensionNode {
    //
}

impl Parent<DirectivesNode> for crate::UnionTypeExtensionNode {
    //
}

impl Parent<DirectivesNode> for crate::EnumTypeExtensionNode {
    //
}

impl Parent<DirectivesNode> for crate::InputObjectTypeExtensionNode {
    //
}

impl Parent<DirectivesNode> for crate::VariableDefinitionNode {
    //
}

impl Parent<DirectivesNode> for crate::SchemaExtensionNode {
    //
}

impl Parent<DirectivesNode> for crate::FieldDefinitionNode {
    //
}

impl DirectivesNode {
    pub fn directives(&self) -> impl Iterator<Item = crate::DirectiveNode> + '_ {
        <Self as Parent<crate::DirectiveNode>>::children(self)
    }
}
