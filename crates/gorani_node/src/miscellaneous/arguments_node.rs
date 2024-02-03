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
