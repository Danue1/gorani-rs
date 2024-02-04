use gorani_span::Span;

pub struct VariableDefinition {
    pub span: Span,
    pub variable: crate::Variable,
    pub r#type: crate::Type,
    pub default_value: Option<crate::Value>,
    pub directives: Vec<crate::Directive>,
}
