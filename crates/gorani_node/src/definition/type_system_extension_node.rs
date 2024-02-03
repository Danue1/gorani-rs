use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeSystemExtensionNode(pub(crate) SyntaxNode);

pub enum TypeSystemExtensionKindNOde {
    TypeExtension(crate::TypeExtensionNode),
    SchemaExtension(crate::SchemaExtensionNode),
}

impl Node for TypeSystemExtensionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_SYSTEM_EXTENSION) {
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

impl Parent<TypeSystemExtensionNode> for crate::TypeSystemDefinitionOrExtensionNode {
    //
}

impl TypeSystemExtensionKindNOde {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_EXTENSION) {
            Some(Self::TypeExtension(crate::TypeExtensionNode::cast(node)?))
        } else if matches!(node.kind(), SyntaxKind::SCHEMA_EXTENSION) {
            Some(Self::SchemaExtension(crate::SchemaExtensionNode::cast(
                node,
            )?))
        } else {
            None
        }
    }
}

impl TypeSystemExtensionNode {
    pub fn kind(&self) -> Option<TypeSystemExtensionKindNOde> {
        self.0
            .children()
            .find_map(TypeSystemExtensionKindNOde::cast)
    }

    pub fn type_extension(&self) -> Option<crate::TypeExtensionNode> {
        <Self as crate::Parent<crate::TypeExtensionNode>>::child(self)
    }

    pub fn schema_extension(&self) -> Option<crate::SchemaExtensionNode> {
        <Self as crate::Parent<crate::SchemaExtensionNode>>::child(self)
    }
}
