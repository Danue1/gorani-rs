use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct InterfaceTypeExtensionNode(pub(crate) SyntaxNode);

impl Node for InterfaceTypeExtensionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::INTERFACE_TYPE_EXTENSION) {
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

impl Parent<InterfaceTypeExtensionNode> for crate::TypeExtensionNode {
    //
}

impl InterfaceTypeExtensionNode {
    pub fn extend(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_EXTEND))
    }

    pub fn interface(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_INTERFACE))
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as crate::Parent<crate::NameNode>>::child(self)
    }

    pub fn implements_interfaces(&self) -> Option<crate::ImplementsInterfacesNode> {
        <Self as crate::Parent<crate::ImplementsInterfacesNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as crate::Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn fields(&self) -> Option<crate::FieldsDefinitionNode> {
        <Self as crate::Parent<crate::FieldsDefinitionNode>>::child(self)
    }
}
