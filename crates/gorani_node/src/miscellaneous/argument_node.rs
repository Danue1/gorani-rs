use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ArgumentNode(pub(crate) SyntaxNode);

impl Node for ArgumentNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::ARGUMENT) {
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

impl Parent<ArgumentNode> for crate::ArgumentsNode {
    //
}

impl ArgumentNode {
    pub fn name(&self) -> Option<crate::NameNode> {
        self.0.children().find_map(crate::NameNode::cast)
    }

    pub fn colon(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_COLON))
    }

    pub fn value(&self) -> Option<crate::ValueNode> {
        self.0.children().find_map(crate::ValueNode::cast)
    }
}
