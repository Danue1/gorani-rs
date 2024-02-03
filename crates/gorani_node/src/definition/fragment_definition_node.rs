use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct FragmentDefinitionNode(pub(crate) SyntaxNode);

impl Node for FragmentDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::FRAGMENT_DEFINITION) {
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

impl Parent<FragmentDefinitionNode> for crate::ExecutableDefinitionNode {
    //
}

impl FragmentDefinitionNode {
    pub fn fragment(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_FRAGMENT))
    }

    pub fn fragment_name(&self) -> Option<crate::FragmentNameNode> {
        <Self as Parent<crate::FragmentNameNode>>::child(self)
    }

    pub fn type_condition(&self) -> Option<crate::TypeConditionNode> {
        <Self as Parent<crate::TypeConditionNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn selection_set(&self) -> Option<crate::SelectionSetNode> {
        <Self as Parent<crate::SelectionSetNode>>::child(self)
    }
}
