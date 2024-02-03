use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct SelectionNode(pub(crate) SyntaxNode);

impl Node for SelectionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::SELECTION) {
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

impl Parent<SelectionNode> for crate::SelectionSetNode {
    //
}

impl SelectionNode {
    pub fn field(&self) -> Option<crate::FieldNode> {
        <Self as crate::Parent<crate::FieldNode>>::child(self)
    }

    pub fn fragment_spread(&self) -> Option<crate::FragmentSpreadNode> {
        <Self as crate::Parent<crate::FragmentSpreadNode>>::child(self)
    }

    pub fn inline_fragment(&self) -> Option<crate::InlineFragmentNode> {
        <Self as crate::Parent<crate::InlineFragmentNode>>::child(self)
    }
}
