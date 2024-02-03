use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeDefinitionNode(pub(crate) SyntaxNode);

pub enum TypeDefinitionKindNode {
    ScalarTypeDefinitionNode(crate::ScalarTypeDefinitionNode),
    ObjectTypeDefinitionNode(crate::ObjectTypeDefinitionNode),
    InterfaceTypeDefinitionNode(crate::InterfaceTypeDefinitionNode),
    UnionTypeDefinitionNode(crate::UnionTypeDefinitionNode),
    EnumTypeDefinitionNode(crate::EnumTypeDefinitionNode),
    InputObjectTypeDefinitionNode(crate::InputObjectTypeDefinitionNode),
}

impl Node for TypeDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_DEFINITION) {
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

impl TypeDefinitionKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(node) = crate::ScalarTypeDefinitionNode::cast(node.clone()) {
            Some(Self::ScalarTypeDefinitionNode(node))
        } else if let Some(node) = crate::ObjectTypeDefinitionNode::cast(node.clone()) {
            Some(Self::ObjectTypeDefinitionNode(node))
        } else if let Some(node) = crate::InterfaceTypeDefinitionNode::cast(node.clone()) {
            Some(Self::InterfaceTypeDefinitionNode(node))
        } else if let Some(node) = crate::UnionTypeDefinitionNode::cast(node.clone()) {
            Some(Self::UnionTypeDefinitionNode(node))
        } else if let Some(node) = crate::EnumTypeDefinitionNode::cast(node.clone()) {
            Some(Self::EnumTypeDefinitionNode(node))
        } else if let Some(node) = crate::InputObjectTypeDefinitionNode::cast(node.clone()) {
            Some(Self::InputObjectTypeDefinitionNode(node))
        } else {
            None
        }
    }
}

impl Parent<TypeDefinitionNode> for crate::TypeSystemDefinitionNode {
    //
}

impl TypeDefinitionNode {
    pub fn kind(&self) -> Option<TypeDefinitionKindNode> {
        self.0.children().find_map(TypeDefinitionKindNode::cast)
    }

    pub fn scalar_type_definition(&self) -> Option<crate::ScalarTypeDefinitionNode> {
        <Self as Parent<crate::ScalarTypeDefinitionNode>>::child(self)
    }

    pub fn object_type_definition(&self) -> Option<crate::ObjectTypeDefinitionNode> {
        <Self as Parent<crate::ObjectTypeDefinitionNode>>::child(self)
    }

    pub fn interface_type_definition(&self) -> Option<crate::InterfaceTypeDefinitionNode> {
        <Self as Parent<crate::InterfaceTypeDefinitionNode>>::child(self)
    }

    pub fn union_type_definition(&self) -> Option<crate::UnionTypeDefinitionNode> {
        <Self as Parent<crate::UnionTypeDefinitionNode>>::child(self)
    }

    pub fn enum_type_definition(&self) -> Option<crate::EnumTypeDefinitionNode> {
        <Self as Parent<crate::EnumTypeDefinitionNode>>::child(self)
    }

    pub fn input_object_type_definition(&self) -> Option<crate::InputObjectTypeDefinitionNode> {
        <Self as Parent<crate::InputObjectTypeDefinitionNode>>::child(self)
    }
}
