use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct OperationDefinitionNode(pub(crate) SyntaxNode);

impl Node for OperationDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::OPERATION_DEFINITION) {
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

impl Parent<OperationDefinitionNode> for crate::ExecutableDefinitionNode {
    //
}

impl OperationDefinitionNode {
    pub fn operation_type(&self) -> Option<crate::OperationTypeNode> {
        <Self as crate::Parent<crate::OperationTypeNode>>::child(self)
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as crate::Parent<crate::NameNode>>::child(self)
    }

    pub fn variable_definitions(&self) -> Option<crate::VariablesDefinitionNode> {
        <Self as crate::Parent<crate::VariablesDefinitionNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as crate::Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn selection_set(&self) -> Option<crate::SelectionSetNode> {
        <Self as crate::Parent<crate::SelectionSetNode>>::child(self)
    }
}
