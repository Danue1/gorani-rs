use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ExecutableDefinitionNode(pub(crate) SyntaxNode);

pub enum ExecutableDefinitionKind {
    OperationDefinition(crate::OperationDefinitionNode),
    FragmentDefinition(crate::FragmentDefinitionNode),
}

impl Node for ExecutableDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::EXECUTABLE_DEFINITION) {
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

impl Parent<ExecutableDefinitionNode> for crate::ExecutableDocumentNode {
    //
}

impl Parent<ExecutableDefinitionNode> for crate::DefinitionNode {
    //
}

impl ExecutableDefinitionKind {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(operation_definition) = crate::OperationDefinitionNode::cast(node.clone()) {
            Some(Self::OperationDefinition(operation_definition))
        } else if let Some(fragment_definition) = crate::FragmentDefinitionNode::cast(node) {
            Some(Self::FragmentDefinition(fragment_definition))
        } else {
            None
        }
    }
}

impl ExecutableDefinitionNode {
    pub fn kind(&self) -> Option<ExecutableDefinitionKind> {
        self.0.children().find_map(ExecutableDefinitionKind::cast)
    }

    pub fn operation_definition(&self) -> Option<crate::OperationDefinitionNode> {
        <Self as crate::Parent<crate::OperationDefinitionNode>>::child(self)
    }

    pub fn fragment_definition(&self) -> Option<crate::FragmentDefinitionNode> {
        <Self as crate::Parent<crate::FragmentDefinitionNode>>::child(self)
    }
}
