use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct FieldsDefinitionNode(pub(crate) SyntaxNode);

impl Node for FieldsDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::FIELDS_DEFINITION) {
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

impl Parent<FieldsDefinitionNode> for crate::ObjectTypeDefinitionNode {
    //
}

impl Parent<FieldsDefinitionNode> for crate::InterfaceTypeDefinitionNode {
    //
}

impl Parent<FieldsDefinitionNode> for crate::ObjectTypeExtensionNode {
    //
}

impl Parent<FieldsDefinitionNode> for crate::InterfaceTypeExtensionNode {
    //
}

impl FieldsDefinitionNode {
    pub fn field_definitions(&self) -> impl Iterator<Item = crate::FieldDefinitionNode> + '_ {
        <Self as Parent<crate::FieldDefinitionNode>>::children(self)
    }
}
