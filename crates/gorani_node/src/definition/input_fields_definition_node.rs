use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct InputFieldsDefinitionNode(pub(crate) SyntaxNode);

impl Node for InputFieldsDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::INPUT_FIELDS_DEFINITION) {
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

impl Parent<InputFieldsDefinitionNode> for crate::InputObjectTypeDefinitionNode {
    //
}

impl Parent<InputFieldsDefinitionNode> for crate::InputObjectTypeExtensionNode {
    //
}

impl InputFieldsDefinitionNode {
    pub fn left_brace(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_LEFT_BRACE))
    }

    pub fn input_value_definitions(
        &self,
    ) -> impl Iterator<Item = crate::InputValueDefinitionNode> + '_ {
        <Self as Parent<crate::InputValueDefinitionNode>>::children(self)
    }

    pub fn right_brace(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_RIGHT_BRACE))
    }
}
