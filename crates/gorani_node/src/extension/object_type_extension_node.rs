use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ObjectTypeExtensionNode(pub(crate) SyntaxNode);

impl Node for ObjectTypeExtensionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::OBJECT_TYPE_EXTENSION) {
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

impl Parent<ObjectTypeExtensionNode> for crate::TypeExtensionNode {
    //
}

impl ObjectTypeExtensionNode {
    pub fn extend(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_EXTEND))
    }

    pub fn r#type(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_TYPE))
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as crate::Parent<crate::NameNode>>::child(self)
    }

    pub fn interfaces(&self) -> Option<crate::ImplementsInterfacesNode> {
        <Self as crate::Parent<crate::ImplementsInterfacesNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as crate::Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn fields_definition(&self) -> Option<crate::FieldsDefinitionNode> {
        <Self as crate::Parent<crate::FieldsDefinitionNode>>::child(self)
    }
}
