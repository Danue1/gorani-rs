use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeSystemDefinitionOrExtensionNode(pub(crate) SyntaxNode);

pub enum TypeSystemDefinitionOrExtensionKindNode {
    TypeSystemDefinition(crate::TypeSystemDefinitionNode),
    TypeSystemExtension(crate::TypeSystemExtensionNode),
}

impl Node for TypeSystemDefinitionOrExtensionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_SYSTEM_DEFINITION_OR_EXTENSION) {
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

impl Parent<TypeSystemDefinitionOrExtensionNode> for crate::DefinitionNode {
    //
}

impl Parent<TypeSystemDefinitionOrExtensionNode> for crate::TypeSystemExtensionDocumentNode {
    //
}

impl TypeSystemDefinitionOrExtensionKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(node) = crate::TypeSystemDefinitionNode::cast(node.clone()) {
            Some(Self::TypeSystemDefinition(node))
        } else if let Some(node) = crate::TypeSystemExtensionNode::cast(node.clone()) {
            Some(Self::TypeSystemExtension(node))
        } else {
            None
        }
    }
}

impl TypeSystemDefinitionOrExtensionNode {
    pub fn kind(&self) -> Option<TypeSystemDefinitionOrExtensionKindNode> {
        self.0
            .children()
            .find_map(TypeSystemDefinitionOrExtensionKindNode::cast)
    }

    pub fn type_system_definition(&self) -> Option<crate::TypeSystemDefinitionNode> {
        <Self as crate::Parent<crate::TypeSystemDefinitionNode>>::child(self)
    }

    pub fn type_system_extension(&self) -> Option<crate::TypeSystemExtensionNode> {
        <Self as crate::Parent<crate::TypeSystemExtensionNode>>::child(self)
    }
}
