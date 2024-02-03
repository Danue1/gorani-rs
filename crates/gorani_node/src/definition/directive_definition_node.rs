use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct DirectiveDefinitionNode(pub(crate) SyntaxNode);

impl Node for DirectiveDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::DIRECTIVE_DEFINITION) {
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

impl Parent<DirectiveDefinitionNode> for crate::TypeSystemDefinitionNode {
    //
}

impl DirectiveDefinitionNode {
    pub fn description(&self) -> Option<crate::DescriptionNode> {
        <Self as Parent<crate::DescriptionNode>>::child(self)
    }

    pub fn directive(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_DIRECTIVE))
    }

    pub fn at(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::SYMBOL_AT))
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }

    pub fn arguments_definition(&self) -> Option<crate::ArgumentsDefinitionNode> {
        <Self as Parent<crate::ArgumentsDefinitionNode>>::child(self)
    }

    pub fn repeatable(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_REPEATABLE))
    }

    pub fn on(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_ON))
    }

    pub fn directive_locations(&self) -> Option<crate::DirectiveLocationsNode> {
        <Self as Parent<crate::DirectiveLocationsNode>>::child(self)
    }
}
