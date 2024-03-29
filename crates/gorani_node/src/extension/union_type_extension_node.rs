use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct UnionTypeExtensionNode(pub(crate) SyntaxNode);

impl Node for UnionTypeExtensionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::UNION_TYPE_EXTENSION) {
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

impl Parent<UnionTypeExtensionNode> for crate::TypeExtensionNode {
    //
}

impl UnionTypeExtensionNode {
    pub fn extend(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_EXTEND))
    }

    pub fn union(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_UNION))
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as crate::Parent<crate::NameNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as crate::Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn union_member_types(&self) -> Option<crate::UnionMemberTypesNode> {
        <Self as crate::Parent<crate::UnionMemberTypesNode>>::child(self)
    }
}
