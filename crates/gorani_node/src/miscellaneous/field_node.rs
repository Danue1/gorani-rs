use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct FieldNode(pub(crate) SyntaxNode);

impl Node for FieldNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::FIELD) {
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

impl Parent<FieldNode> for crate::SelectionNode {
    //
}

impl FieldNode {
    pub fn alias(&self) -> Option<crate::AliasNode> {
        <Self as crate::Parent<crate::AliasNode>>::child(self)
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as crate::Parent<crate::NameNode>>::child(self)
    }

    pub fn arguments(&self) -> Option<crate::ArgumentsNode> {
        <Self as crate::Parent<crate::ArgumentsNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as crate::Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn selection_set(&self) -> Option<crate::SelectionSetNode> {
        <Self as crate::Parent<crate::SelectionSetNode>>::child(self)
    }
}
