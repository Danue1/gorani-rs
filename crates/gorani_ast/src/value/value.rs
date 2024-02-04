use gorani_span::Span;

pub struct Value {
    pub span: Span,
    pub kind: ValueKind,
}

pub enum ValueKind {
    Variable(crate::Variable),
    IntValue(crate::IntValue),
    FloatValue(crate::FloatValue),
    StringValue(crate::StringValue),
    BooleanValue(crate::BooleanValue),
    NullValue(crate::NullValue),
    EnumValue(crate::EnumValue),
    ListValue(crate::ListValue),
    ObjectValue(crate::ObjectValue),
}
