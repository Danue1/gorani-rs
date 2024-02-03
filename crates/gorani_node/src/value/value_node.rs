use crate::{Node, Parent};
use gorani_syntax::{SyntaxKind, SyntaxNode};

pub struct ValueNode(pub(crate) SyntaxNode);

pub enum ValueKindNode {
    Variable(crate::VariableNode),
    IntValue(crate::IntValueNode),
    FloatValue(crate::FloatValueNode),
    StringValue(crate::StringValueNode),
    BooleanValue(crate::BooleanValueNode),
    NullValue(crate::NullValueNode),
    EnumValue(crate::EnumValueNode),
    ListValue(crate::ListValueNode),
    ObjectValue(crate::ObjectValueNode),
}

impl Node for ValueNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if matches!(node.kind(), SyntaxKind::VALUE) {
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

impl Parent<ValueNode> for crate::ArgumentNode {
    //
}

impl Parent<ValueNode> for crate::ListValueNode {
    //
}

impl Parent<ValueNode> for crate::ObjectFieldNode {
    //
}

impl Parent<ValueNode> for crate::DefaultValueNode {
    //
}

impl ValueKindNode {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(node) = crate::VariableNode::cast(node.clone()) {
            Some(ValueKindNode::Variable(node))
        } else if let Some(node) = crate::IntValueNode::cast(node.clone()) {
            Some(ValueKindNode::IntValue(node))
        } else if let Some(node) = crate::FloatValueNode::cast(node.clone()) {
            Some(ValueKindNode::FloatValue(node))
        } else if let Some(node) = crate::StringValueNode::cast(node.clone()) {
            Some(ValueKindNode::StringValue(node))
        } else if let Some(node) = crate::BooleanValueNode::cast(node.clone()) {
            Some(ValueKindNode::BooleanValue(node))
        } else if let Some(node) = crate::NullValueNode::cast(node.clone()) {
            Some(ValueKindNode::NullValue(node))
        } else if let Some(node) = crate::EnumValueNode::cast(node.clone()) {
            Some(ValueKindNode::EnumValue(node))
        } else if let Some(node) = crate::ListValueNode::cast(node.clone()) {
            Some(ValueKindNode::ListValue(node))
        } else if let Some(node) = crate::ObjectValueNode::cast(node) {
            Some(ValueKindNode::ObjectValue(node))
        } else {
            None
        }
    }
}

impl ValueNode {
    pub fn kind(&self) -> Option<ValueKindNode> {
        self.node().children().find_map(ValueKindNode::cast)
    }

    pub fn variable(&self) -> Option<crate::VariableNode> {
        <Self as crate::Parent<crate::VariableNode>>::child(self)
    }

    pub fn int_value(&self) -> Option<crate::IntValueNode> {
        <Self as crate::Parent<crate::IntValueNode>>::child(self)
    }

    pub fn float_value(&self) -> Option<crate::FloatValueNode> {
        <Self as crate::Parent<crate::FloatValueNode>>::child(self)
    }

    pub fn string_value(&self) -> Option<crate::StringValueNode> {
        <Self as crate::Parent<crate::StringValueNode>>::child(self)
    }

    pub fn boolean_value(&self) -> Option<crate::BooleanValueNode> {
        <Self as crate::Parent<crate::BooleanValueNode>>::child(self)
    }

    pub fn null_value(&self) -> Option<crate::NullValueNode> {
        <Self as crate::Parent<crate::NullValueNode>>::child(self)
    }

    pub fn enum_value(&self) -> Option<crate::EnumValueNode> {
        <Self as crate::Parent<crate::EnumValueNode>>::child(self)
    }

    pub fn list_value(&self) -> Option<crate::ListValueNode> {
        <Self as crate::Parent<crate::ListValueNode>>::child(self)
    }

    pub fn object_value(&self) -> Option<crate::ObjectValueNode> {
        <Self as crate::Parent<crate::ObjectValueNode>>::child(self)
    }
}
