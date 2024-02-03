use crate::{DocumentNode, Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DefinitionNode(pub(crate) SyntaxNode);

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

impl DefinitionNode {
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
