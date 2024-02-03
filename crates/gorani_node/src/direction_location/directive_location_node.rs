use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DirectiveLocationNode(pub(crate) SyntaxNode);

pub enum DirectiveLocationKindNode {
    ExecutableDirectiveLocation(crate::ExecutableDirectiveLocationNode),
    TypeSystemDirectiveLocation(crate::TypeSystemDirectiveLocationNode),
}

impl Node for DirectiveLocationNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DIRECTIVE_LOCATION) {
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

impl Parent<DirectiveLocationNode> for crate::DirectiveLocationsNode {
    //
}

impl DirectiveLocationKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(node) = crate::ExecutableDirectiveLocationNode::cast(node.clone()) {
            Some(Self::ExecutableDirectiveLocation(node))
        } else if let Some(node) = crate::TypeSystemDirectiveLocationNode::cast(node.clone()) {
            Some(Self::TypeSystemDirectiveLocation(node))
        } else {
            None
        }
    }
}

impl DirectiveLocationNode {
    pub fn kind(&self) -> Option<DirectiveLocationKindNode> {
        self.0.children().find_map(DirectiveLocationKindNode::cast)
    }

    pub fn executable_directive_location(&self) -> Option<crate::ExecutableDirectiveLocationNode> {
        <Self as Parent<crate::ExecutableDirectiveLocationNode>>::child(self)
    }

    pub fn type_system_directive_location(&self) -> Option<crate::TypeSystemDirectiveLocationNode> {
        <Self as Parent<crate::TypeSystemDirectiveLocationNode>>::child(self)
    }
}
