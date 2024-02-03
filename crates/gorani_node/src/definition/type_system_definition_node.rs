use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct TypeSystemDefinitionNode(pub(crate) SyntaxNode);

pub enum TypeSystemDefinitionKindNode {
    SchemaDefinition(crate::SchemaDefinitionNode),
    TypeDefinition(crate::TypeDefinitionNode),
    DirectiveDefinition(crate::DirectiveDefinitionNode),
}

impl Node for TypeSystemDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::TYPE_SYSTEM_DEFINITION) {
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

impl Parent<TypeSystemDefinitionNode> for crate::TypeSystemDefinitionOrExtensionNode {
    //
}

impl Parent<TypeSystemDefinitionNode> for crate::TypeSystemDocumentNode {
    //
}

impl TypeSystemDefinitionKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(node) = crate::SchemaDefinitionNode::cast(node.clone()) {
            Some(Self::SchemaDefinition(node))
        } else if let Some(node) = crate::TypeDefinitionNode::cast(node.clone()) {
            Some(Self::TypeDefinition(node))
        } else if let Some(node) = crate::DirectiveDefinitionNode::cast(node.clone()) {
            Some(Self::DirectiveDefinition(node))
        } else {
            None
        }
    }
}

impl TypeSystemDefinitionNode {
    pub fn kind(&self) -> Option<TypeSystemDefinitionKindNode> {
        self.0
            .children()
            .find_map(TypeSystemDefinitionKindNode::cast)
    }

    pub fn schema_definition(&self) -> Option<crate::SchemaDefinitionNode> {
        <Self as crate::Parent<crate::SchemaDefinitionNode>>::child(self)
    }

    pub fn type_definition(&self) -> Option<crate::TypeDefinitionNode> {
        <Self as crate::Parent<crate::TypeDefinitionNode>>::child(self)
    }

    pub fn directive_definition(&self) -> Option<crate::DirectiveDefinitionNode> {
        <Self as crate::Parent<crate::DirectiveDefinitionNode>>::child(self)
    }
}
