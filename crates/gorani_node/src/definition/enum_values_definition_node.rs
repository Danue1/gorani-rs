use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct EnumValuesDefinitionNode(pub(crate) SyntaxNode);

impl Node for EnumValuesDefinitionNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::ENUM_VALUES_DEFINITION) {
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

impl Parent<EnumValuesDefinitionNode> for crate::EnumTypeDefinitionNode {
    //
}

impl Parent<EnumValuesDefinitionNode> for crate::EnumTypeExtensionNode {
    //
}

impl EnumValuesDefinitionNode {
    pub fn enum_value_definitions(
        &self,
    ) -> impl Iterator<Item = crate::EnumValueDefinitionNode> + '_ {
        <Self as Parent<crate::EnumValueDefinitionNode>>::children(self)
    }
}
