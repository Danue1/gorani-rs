use crate::{DocumentNode, Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DefinitionNode(pub(crate) SyntaxNode);

pub enum DefinitionKindNode {
    ExecutableDefinition(crate::ExecutableDefinitionNode),
    TypeSystemDefinitionOrExtension(crate::TypeSystemDefinitionOrExtensionNode),
}

impl Node for DefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DEFINITION) {
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

impl Parent<DefinitionNode> for DocumentNode {
    //
}

impl DefinitionKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(node) = crate::ExecutableDefinitionNode::cast(node.clone()) {
            Some(Self::ExecutableDefinition(node))
        } else if let Some(node) = crate::TypeSystemDefinitionOrExtensionNode::cast(node) {
            Some(Self::TypeSystemDefinitionOrExtension(node))
        } else {
            None
        }
    }
}

impl DefinitionNode {
    pub fn kind(&self) -> Option<DefinitionKindNode> {
        self.0.children().find_map(DefinitionKindNode::cast)
    }

    pub fn executable_definitions(
        &self,
    ) -> impl Iterator<Item = crate::ExecutableDefinitionNode> + '_ {
        <Self as crate::Parent<crate::ExecutableDefinitionNode>>::children(self)
    }

    pub fn type_system_definition_or_extension(
        &self,
    ) -> impl Iterator<Item = crate::TypeSystemDefinitionOrExtensionNode> + '_ {
        <Self as crate::Parent<crate::TypeSystemDefinitionOrExtensionNode>>::children(self)
    }
}
