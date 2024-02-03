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
    pub fn input_value_definitions(
        &self,
    ) -> impl Iterator<Item = crate::InputValueDefinitionNode> + '_ {
        <Self as Parent<crate::InputValueDefinitionNode>>::children(self)
    }
}
