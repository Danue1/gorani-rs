use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct VariablesDefinitionNode(pub(crate) SyntaxNode);

impl Node for VariablesDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::VARIABLES_DEFINITION) {
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

impl Parent<VariablesDefinitionNode> for crate::OperationDefinitionNode {
    //
}

impl VariablesDefinitionNode {
    pub fn left_parens(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_LEFT_PARENS))
    }

    pub fn variable_definitions(&self) -> impl Iterator<Item = crate::VariableDefinitionNode> + '_ {
        <Self as crate::Parent<crate::VariableDefinitionNode>>::children(self)
    }

    pub fn right_parens(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_RIGHT_PARENS))
    }
}
