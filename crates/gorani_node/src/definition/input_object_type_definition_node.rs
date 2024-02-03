use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct InputObjectTypeDefinitionNode(pub(crate) SyntaxNode);

impl Node for InputObjectTypeDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::INPUT_OBJECT_TYPE_DEFINITION) {
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

impl Parent<InputObjectTypeDefinitionNode> for crate::TypeDefinitionNode {
    //
}

impl InputObjectTypeDefinitionNode {
    pub fn description(&self) -> Option<crate::DescriptionNode> {
        <Self as Parent<crate::DescriptionNode>>::child(self)
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn input_fields_definition(&self) -> Option<crate::InputFieldsDefinitionNode> {
        <Self as Parent<crate::InputFieldsDefinitionNode>>::child(self)
    }
}
