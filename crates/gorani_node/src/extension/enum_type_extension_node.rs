use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct EnumTypeExtensionNode(pub(crate) SyntaxNode);

impl Node for EnumTypeExtensionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::ENUM_TYPE_EXTENSION) {
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

impl Parent<EnumTypeExtensionNode> for crate::TypeExtensionNode {
    //
}

impl EnumTypeExtensionNode {
    pub fn extend(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_EXTEND))
    }

    pub fn r#enum(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_ENUM))
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as crate::Parent<crate::NameNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as crate::Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn enum_values_definition(&self) -> Option<crate::EnumValuesDefinitionNode> {
        <Self as crate::Parent<crate::EnumValuesDefinitionNode>>::child(self)
    }
}
