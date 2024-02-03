use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct SelectionSetNode(pub(crate) SyntaxNode);

impl Node for SelectionSetNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::SELECTION_SET) {
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

impl Parent<SelectionSetNode> for crate::OperationDefinitionNode {
    //
}

impl Parent<SelectionSetNode> for crate::FieldNode {
    //
}

impl Parent<SelectionSetNode> for crate::InlineFragmentNode {
    //
}

impl Parent<SelectionSetNode> for crate::FragmentDefinitionNode {
    //
}

impl SelectionSetNode {
    pub fn selections(&self) -> impl Iterator<Item = crate::SelectionNode> + '_ {
        <Self as crate::Parent<crate::SelectionNode>>::children(self)
    }
}
