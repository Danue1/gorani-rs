use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DirectiveNode(pub(crate) SyntaxNode);

impl Node for DirectiveNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DIRECTIVE) {
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

impl Parent<DirectiveNode> for crate::DirectivesNode {
    //
}

impl DirectiveNode {
    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }

    pub fn arguments(&self) -> Option<crate::ArgumentsNode> {
        <Self as Parent<crate::ArgumentsNode>>::child(self)
    }
}
