use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ExecutableDirectiveLocationNode(pub(crate) SyntaxNode);

#[allow(non_camel_case_types)]
pub enum ExecutableDirectiveLocationKind {
    QUERY,
    MUTATION,
    SUBSCRIPTION,
    FIELD,
    FRAGMENT_DEFINITION,
    FRAGMENT_SPREAD,
    INLINE_FRAGMENT,
    VARIABLE_DEFINITION,
}

impl Node for ExecutableDirectiveLocationNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::EXECUTABLE_DIRECTIVE_LOCATION) {
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

impl Parent<ExecutableDirectiveLocationNode> for crate::DirectiveLocationNode {
    //
}

impl ExecutableDirectiveLocationNode {
    pub fn kind(&self) -> Option<ExecutableDirectiveLocationKind> {
        match self.0.first_child() {
            Some(child) => match child.kind() {
                SyntaxKind::LOCATION_QUERY => Some(ExecutableDirectiveLocationKind::QUERY),
                SyntaxKind::LOCATION_MUTATION => Some(ExecutableDirectiveLocationKind::MUTATION),
                SyntaxKind::LOCATION_SUBSCRIPTION => {
                    Some(ExecutableDirectiveLocationKind::SUBSCRIPTION)
                }
                SyntaxKind::LOCATION_FIELD => Some(ExecutableDirectiveLocationKind::FIELD),
                SyntaxKind::LOCATION_FRAGMENT_DEFINITION => {
                    Some(ExecutableDirectiveLocationKind::FRAGMENT_DEFINITION)
                }
                SyntaxKind::LOCATION_FRAGMENT_SPREAD => {
                    Some(ExecutableDirectiveLocationKind::FRAGMENT_SPREAD)
                }
                SyntaxKind::LOCATION_INLINE_FRAGMENT => {
                    Some(ExecutableDirectiveLocationKind::INLINE_FRAGMENT)
                }
                SyntaxKind::LOCATION_VARIABLE_DEFINITION => {
                    Some(ExecutableDirectiveLocationKind::VARIABLE_DEFINITION)
                }
                _ => None,
            },
            _ => None,
        }
    }
}
