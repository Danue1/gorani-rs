use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct OperationTypeNode(pub(crate) SyntaxNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

impl Node for OperationTypeNode {
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

impl Parent<OperationTypeNode> for crate::OperationDefinitionNode {
    //
}

impl Parent<OperationTypeNode> for crate::RootOperationTypeDefinitionNode {
    //
}

impl OperationTypeNode {
    pub fn operation_type(&self) -> Option<OperationType> {
        match self.node().kind() {
            SyntaxKind::SYMBOL_QUERY => Some(OperationType::Query),
            SyntaxKind::SYMBOL_MUTATION => Some(OperationType::Mutation),
            SyntaxKind::SYMBOL_SUBSCRIPTION => Some(OperationType::Subscription),
            _ => None,
        }
    }
}
