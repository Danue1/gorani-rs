use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ObjectTypeDefinitionNode(pub(crate) SyntaxNode);

impl Node for ObjectTypeDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::OBJECT_TYPE_DEFINITION) {
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

impl Parent<ObjectTypeDefinitionNode> for crate::TypeDefinitionNode {
    //
}

impl ObjectTypeDefinitionNode {
    pub fn description(&self) -> Option<crate::DescriptionNode> {
        <Self as Parent<crate::DescriptionNode>>::child(self)
    }

    pub fn r#type(&self) -> Option<SyntaxNode> {
        self.0
            .children()
            .find(|node| matches!(node.kind(), SyntaxKind::KEYWORD_TYPE))
    }

    pub fn name(&self) -> Option<crate::NameNode> {
        <Self as Parent<crate::NameNode>>::child(self)
    }

    pub fn implements_interfaces(&self) -> Option<crate::ImplementsInterfacesNode> {
        <Self as Parent<crate::ImplementsInterfacesNode>>::child(self)
    }

    pub fn directives(&self) -> Option<crate::DirectivesNode> {
        <Self as Parent<crate::DirectivesNode>>::child(self)
    }

    pub fn fields_definition(&self) -> Option<crate::FieldsDefinitionNode> {
        <Self as Parent<crate::FieldsDefinitionNode>>::child(self)
    }
}
