use crate::{ListTypeNode, Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeNode(pub(crate) SyntaxNode);

pub enum TypeKindNode {
    NamedType(crate::NamedTypeNode),
    ListType(crate::ListTypeNode),
    NonNullType(crate::NonNullTypeNode),
}

impl Node for TypeNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE) {
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

impl Parent<crate::TypeNode> for ListTypeNode {
    //
}

impl Parent<crate::TypeNode> for crate::FieldDefinitionNode {
    //
}

impl Parent<crate::TypeNode> for crate::InputValueDefinitionNode {
    //
}

impl Parent<crate::TypeNode> for crate::VariableDefinitionNode {
    //
}

impl TypeKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if crate::NamedTypeNode::cast(node.clone()).is_some() {
            Some(Self::NamedType(crate::NamedTypeNode::cast(node)?))
        } else if crate::ListTypeNode::cast(node.clone()).is_some() {
            Some(Self::ListType(crate::ListTypeNode::cast(node)?))
        } else if crate::NonNullTypeNode::cast(node.clone()).is_some() {
            Some(Self::NonNullType(crate::NonNullTypeNode::cast(node)?))
        } else {
            None
        }
    }
}

impl TypeNode {
    pub fn kind(&self) -> Option<TypeKindNode> {
        self.0.children().find_map(TypeKindNode::cast)
    }

    pub fn named_type(&self) -> Option<crate::NamedTypeNode> {
        <Self as Parent<crate::NamedTypeNode>>::child(self)
    }

    pub fn list_type(&self) -> Option<crate::ListTypeNode> {
        <Self as Parent<crate::ListTypeNode>>::child(self)
    }

    pub fn non_null_type(&self) -> Option<crate::NonNullTypeNode> {
        <Self as Parent<crate::NonNullTypeNode>>::child(self)
    }
}
