use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct UnionMemberTypesNode(pub(crate) SyntaxNode);

impl Node for UnionMemberTypesNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::UNION_MEMBER_TYPES) {
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

impl Parent<UnionMemberTypesNode> for crate::UnionTypeDefinitionNode {
    //
}

impl Parent<UnionMemberTypesNode> for crate::UnionTypeExtensionNode {
    //
}

impl UnionMemberTypesNode {
    pub fn equal(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_EQUAL))
    }

    pub fn pipes(&self) -> impl Iterator<Item = SyntaxNode> + '_ {
        self.0
            .children()
            .filter(|node| matches!(node.kind(), SyntaxKind::SYMBOL_PIPE))
    }

    pub fn named_types(&self) -> Option<crate::NamedTypeNode> {
        <Self as Parent<crate::NamedTypeNode>>::child(self)
    }
}
