use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct SelectionNode(pub(crate) SyntaxNode);

pub enum SelectionKindNode {
    Field(crate::FieldNode),
    FragmentSpread(crate::FragmentSpreadNode),
    InlineFragment(crate::InlineFragmentNode),
}

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

impl SelectionKindNode {
    pub fn cast(node: SyntaxNode) -> Option<SelectionKindNode> {
        if let Some(field) = crate::FieldNode::cast(node.clone()) {
            Some(Self::Field(field))
        } else if let Some(fragment_spread) = crate::FragmentSpreadNode::cast(node.clone()) {
            Some(Self::FragmentSpread(fragment_spread))
        } else if let Some(inline_fragment) = crate::InlineFragmentNode::cast(node.clone()) {
            Some(Self::InlineFragment(inline_fragment))
        } else {
            None
        }
    }
}

impl SelectionNode {
    pub fn kind(&self) -> Option<SelectionKindNode> {
        self.0.children().find_map(SelectionKindNode::cast)
    }

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
