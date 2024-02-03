use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeExtensionNode(pub(crate) SyntaxNode);

pub enum TypeExtensionKindNode {
    ScalarTypeExtension(crate::ScalarTypeExtensionNode),
    ObjectTypeExtension(crate::ObjectTypeExtensionNode),
    InterfaceTypeExtension(crate::InterfaceTypeExtensionNode),
    UnionTypeExtension(crate::UnionTypeExtensionNode),
    EnumTypeExtension(crate::EnumTypeExtensionNode),
    InputObjectTypeExtension(crate::InputObjectTypeExtensionNode),
}

impl Node for TypeExtensionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_EXTENSION) {
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

impl Parent<TypeExtensionNode> for crate::TypeSystemExtensionNode {
    //
}

impl TypeExtensionKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(node) = crate::ScalarTypeExtensionNode::cast(node.clone()) {
            Some(Self::ScalarTypeExtension(node))
        } else if let Some(node) = crate::ObjectTypeExtensionNode::cast(node.clone()) {
            Some(Self::ObjectTypeExtension(node))
        } else if let Some(node) = crate::InterfaceTypeExtensionNode::cast(node.clone()) {
            Some(Self::InterfaceTypeExtension(node))
        } else if let Some(node) = crate::UnionTypeExtensionNode::cast(node.clone()) {
            Some(Self::UnionTypeExtension(node))
        } else if let Some(node) = crate::EnumTypeExtensionNode::cast(node.clone()) {
            Some(Self::EnumTypeExtension(node))
        } else if let Some(node) = crate::InputObjectTypeExtensionNode::cast(node.clone()) {
            Some(Self::InputObjectTypeExtension(node))
        } else {
            None
        }
    }
}

impl TypeExtensionNode {
    pub fn kind(&self) -> Option<TypeExtensionKindNode> {
        self.0.children().find_map(TypeExtensionKindNode::cast)
    }

    pub fn scalar_type_extension(&self) -> Option<crate::ScalarTypeExtensionNode> {
        <Self as crate::Parent<crate::ScalarTypeExtensionNode>>::child(self)
    }

    pub fn object_type_extension(&self) -> Option<crate::ObjectTypeExtensionNode> {
        <Self as crate::Parent<crate::ObjectTypeExtensionNode>>::child(self)
    }

    pub fn interface_type_extension(&self) -> Option<crate::InterfaceTypeExtensionNode> {
        <Self as crate::Parent<crate::InterfaceTypeExtensionNode>>::child(self)
    }

    pub fn union_type_extension(&self) -> Option<crate::UnionTypeExtensionNode> {
        <Self as crate::Parent<crate::UnionTypeExtensionNode>>::child(self)
    }

    pub fn enum_type_extension(&self) -> Option<crate::EnumTypeExtensionNode> {
        <Self as crate::Parent<crate::EnumTypeExtensionNode>>::child(self)
    }

    pub fn input_object_type_extension(&self) -> Option<crate::InputObjectTypeExtensionNode> {
        <Self as crate::Parent<crate::InputObjectTypeExtensionNode>>::child(self)
    }
}
