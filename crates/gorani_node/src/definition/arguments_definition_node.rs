use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ArgumentsDefinitionNode(pub(crate) SyntaxNode);

impl Node for ArgumentsDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::ARGUMENTS_DEFINITION) {
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

impl Parent<ArgumentsDefinitionNode> for crate::FieldDefinitionNode {
    //
}

impl Parent<ArgumentsDefinitionNode> for crate::DirectiveDefinitionNode {
    //
}

impl ArgumentsDefinitionNode {
    pub fn left_parens(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_LEFT_PARENS))
    }

    pub fn input_value_definitions(
        &self,
    ) -> impl Iterator<Item = crate::InputValueDefinitionNode> + '_ {
        <Self as Parent<crate::InputValueDefinitionNode>>::children(self)
    }

    pub fn right_parens(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_RIGHT_PARENS))
    }
}
