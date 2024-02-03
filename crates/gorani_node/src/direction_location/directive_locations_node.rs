use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DirectiveLocationsNode(pub(crate) SyntaxNode);

impl Node for DirectiveLocationsNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DIRECTIVE_LOCATIONS) {
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

impl Parent<DirectiveLocationsNode> for crate::DirectiveDefinitionNode {
    //
}

impl DirectiveLocationsNode {
    pub fn directive_location(&self) -> impl Iterator<Item = crate::DirectiveLocationNode> + '_ {
        <Self as Parent<crate::DirectiveLocationNode>>::children(self)
    }
}
