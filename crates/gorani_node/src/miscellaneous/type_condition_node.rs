use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeConditionNode(pub(crate) SyntaxNode);

impl Node for TypeConditionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_CONDITION) {
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

impl Parent<TypeConditionNode> for crate::InlineFragmentNode {
    //
}

impl Parent<TypeConditionNode> for crate::FragmentDefinitionNode {
    //
}

impl TypeConditionNode {
    pub fn named_type(&self) -> Option<crate::NamedTypeNode> {
        <Self as Parent<crate::NamedTypeNode>>::child(self)
    }
}
