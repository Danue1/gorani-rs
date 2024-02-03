use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ImplementsInterfacesNode(pub(crate) SyntaxNode);

impl Node for ImplementsInterfacesNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::IMPLEMENTS_INTERFACES) {
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

impl Parent<ImplementsInterfacesNode> for crate::ObjectTypeDefinitionNode {
    //
}

impl Parent<ImplementsInterfacesNode> for crate::InterfaceTypeDefinitionNode {
    //
}

impl Parent<ImplementsInterfacesNode> for crate::ObjectTypeExtensionNode {
    //
}

impl Parent<ImplementsInterfacesNode> for crate::InterfaceTypeExtensionNode {
    //
}

impl ImplementsInterfacesNode {
    pub fn implements(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_IMPLEMENTS))
    }

    pub fn ampersands(&self) -> impl Iterator<Item = SyntaxNode> {
        self.0
            .children()
            .filter(|node| matches!(node.kind(), SyntaxKind::SYMBOL_AMPERSAND))
    }

    pub fn named_types(&self) -> Option<crate::NamedTypeNode> {
        <Self as Parent<crate::NamedTypeNode>>::child(self)
    }
}
