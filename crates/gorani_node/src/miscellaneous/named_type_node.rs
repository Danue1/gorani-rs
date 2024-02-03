use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct NamedTypeNode(pub(crate) SyntaxNode);

impl Node for NamedTypeNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::NAMED_TYPE) {
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

impl Parent<NamedTypeNode> for crate::TypeConditionNode {
    //
}

impl Parent<NamedTypeNode> for crate::TypeNode {
    //
}

impl Parent<NamedTypeNode> for crate::NonNullTypeNode {
    //
}

impl Parent<NamedTypeNode> for crate::RootOperationTypeDefinitionNode {
    //
}

impl Parent<NamedTypeNode> for crate::ImplementsInterfacesNode {
    //
}

impl Parent<NamedTypeNode> for crate::UnionMemberTypesNode {
    //
}

impl NamedTypeNode {
    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }
}
