use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct VariableDefinitionNode(pub(crate) SyntaxNode);

impl Node for VariableDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::VARIABLE_DEFINITION) {
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

impl Parent<VariableDefinitionNode> for crate::VariablesDefinitionNode {
    //
}

impl VariableDefinitionNode {
    pub fn variable(&self) -> Option<crate::VariableNode> {
        <Self as crate::Parent<crate::VariableNode>>::child(self)
    }

    pub fn colon(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_COLON))
    }

    pub fn r#type(&self) -> Option<crate::TypeNode> {
        <Self as crate::Parent<crate::TypeNode>>::child(self)
    }

    pub fn default_value(&self) -> Option<crate::DefaultValueNode> {
        <Self as crate::Parent<crate::DefaultValueNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as crate::Parent<crate::DirectivesNode>>::child(self)
    }
}
