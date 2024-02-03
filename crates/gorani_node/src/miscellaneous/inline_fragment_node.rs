use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct InlineFragmentNode(pub(crate) SyntaxNode);

impl Node for InlineFragmentNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::INLINE_FRAGMENT) {
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

impl Parent<InlineFragmentNode> for crate::SelectionNode {
    //
}

impl InlineFragmentNode {
    pub fn spread(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_SPREAD))
    }

    pub fn type_condition(&self) -> Option<crate::TypeConditionNode> {
        <Self as Parent<crate::TypeConditionNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn selection_set(&self) -> Option<crate::SelectionSetNode> {
        <Self as Parent<crate::SelectionSetNode>>::child(self)
    }
}
