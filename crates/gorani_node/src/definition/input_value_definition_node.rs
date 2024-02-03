use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct InputValueDefinitionNode(pub(crate) SyntaxNode);

impl Node for InputValueDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::INPUT_VALUE_DEFINITION) {
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

impl Parent<InputValueDefinitionNode> for crate::ArgumentsDefinitionNode {
    //
}

impl Parent<InputValueDefinitionNode> for crate::InputFieldsDefinitionNode {
    //
}

impl InputValueDefinitionNode {
    pub fn description(&self) -> Option<crate::DescriptionNode> {
        <Self as Parent<crate::DescriptionNode>>::child(self)
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }

    pub fn colon(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_COLON))
    }

    pub fn r#type(&self) -> Option<crate::TypeNode> {
        <Self as Parent<crate::TypeNode>>::child(self)
    }

    pub fn default_value(&self) -> Option<crate::DefaultValueNode> {
        <Self as Parent<crate::DefaultValueNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as Parent<crate::DirectivesNode>>::child(self)
    }
}
