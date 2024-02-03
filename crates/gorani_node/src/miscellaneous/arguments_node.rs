use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ArgumentsNode(pub(crate) SyntaxNode);

impl Node for ArgumentsNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::ARGUMENTS) {
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

impl Parent<ArgumentsNode> for crate::FieldNode {
    //
}

impl Parent<ArgumentsNode> for crate::DirectiveNode {
    //
}

impl ArgumentsNode {
    pub fn left_parens(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_LEFT_PARENS))
    }

    pub fn arguments(&self) -> impl Iterator<Item = crate::ArgumentNode> + '_ {
        <Self as crate::Parent<crate::ArgumentNode>>::children(self)
    }

    pub fn right_parens(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_RIGHT_PARENS))
    }
}
