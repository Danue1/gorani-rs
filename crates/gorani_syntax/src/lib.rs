pub type SyntaxNode = rowan::SyntaxNode<SyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    ERROR,

    /// https://spec.graphql.org/draft/#sec-Document
    DOCUMENT,

    /// https://spec.graphql.org/draft/#ExecutableDocument
    EXECUTABLE_DOCUMENT,

    /// https://spec.graphql.org/draft/#TypeSystemDocument
    TYPE_SYSTEM_DOCUMENT,

    /// https://spec.graphql.org/draft/#TypeSystemExtensionDocument
    TYPE_SYSTEM_EXTENSION_DOCUMENT,

    /// https://spec.graphql.org/draft/#Definition
    DEFINITION,

    /// https://spec.graphql.org/draft/#ExecutableDefinition
    EXECUTABLE_DEFINITION,

    /// https://spec.graphql.org/draft/#SchemaDefinition
    SCHEMA_DEFINITION,

    /// https://spec.graphql.org/draft/#TypeSystemDefinition
    TYPE_SYSTEM_DEFINITION,

    /// https://spec.graphql.org/draft/#TypeSystemDefinitionOrExtension
    TYPE_SYSTEM_DEFINITION_OR_EXTENSION,

    /// https://spec.graphql.org/draft/#OperationDefinition
    OPERATION_DEFINITION,

    /// https://spec.graphql.org/draft/#FragmentDefinition
    FRAGMENT_DEFINITION,

    /// https://spec.graphql.org/draft/#VariablesDefinition
    VARIABLES_DEFINITION,

    /// https://spec.graphql.org/draft/#VariableDefinition
    VARIABLE_DEFINITION,

    /// https://spec.graphql.org/draft/#RootOperationTypeDefinition
    ROOT_OPERATION_TYPE_DEFINITION,

    /// https://spec.graphql.org/draft/#DirectiveDefinition
    DIRECTIVE_DEFINITION,

    /// https://spec.graphql.org/draft/#ArgumentsDefinition
    ARGUMENTS_DEFINITION,

    /// https://spec.graphql.org/draft/#InputValueDefinition
    INPUT_VALUE_DEFINITION,

    /// https://spec.graphql.org/draft/#InputFieldsDefinition
    INPUT_FIELDS_DEFINITION,

    /// https://spec.graphql.org/draft/#FieldsDefinition
    FIELDS_DEFINITION,

    /// https://spec.graphql.org/draft/#FieldDefinition
    FIELD_DEFINITION,

    /// https://spec.graphql.org/draft/#EnumValuesDefinition
    ENUM_VALUES_DEFINITION,

    /// https://spec.graphql.org/draft/#EnumValueDefinition
    ENUM_VALUE_DEFINITION,

    /// https://spec.graphql.org/draft/#TypeDefinition
    TYPE_DEFINITION,

    /// https://spec.graphql.org/draft/#ScalarTypeDefinition
    SCALAR_TYPE_DEFINITION,

    /// https://spec.graphql.org/draft/#ObjectTypeDefinition
    OBJECT_TYPE_DEFINITION,

    /// https://spec.graphql.org/draft/#InterfaceTypeDefinition
    INTERFACE_TYPE_DEFINITION,

    /// https://spec.graphql.org/draft/#UnionTypeDefinition
    UNION_TYPE_DEFINITION,

    /// https://spec.graphql.org/draft/#EnumTypeDefinition
    ENUM_TYPE_DEFINITION,

    /// https://spec.graphql.org/draft/#InputObjectTypeDefinition
    INPUT_OBJECT_TYPE_DEFINITION,

    /// https://spec.graphql.org/draft/#TypeSystemExtension
    TYPE_SYSTEM_EXTENSION,

    /// https://spec.graphql.org/draft/#SchemaExtension
    SCHEMA_EXTENSION,

    /// https://spec.graphql.org/draft/#TypeExtension
    TYPE_EXTENSION,

    /// https://spec.graphql.org/draft/#ScalarTypeExtension
    SCALAR_TYPE_EXTENSION,

    /// https://spec.graphql.org/draft/#ObjectTypeExtension
    OBJECT_TYPE_EXTENSION,

    /// https://spec.graphql.org/draft/#InterfaceTypeExtension
    INTERFACE_TYPE_EXTENSION,

    /// https://spec.graphql.org/draft/#UnionTypeExtension
    UNION_TYPE_EXTENSION,

    /// https://spec.graphql.org/draft/#EnumTypeExtension
    ENUM_TYPE_EXTENSION,

    /// https://spec.graphql.org/draft/#InputObjectTypeExtension
    INPUT_OBJECT_TYPE_EXTENSION,

    /// https://spec.graphql.org/draft/#UnionMemberTypes
    UNION_MEMBER_TYPES,

    /// https://spec.graphql.org/draft/#ImplementsInterfaces
    IMPLEMENTS_INTERFACES,

    /// https://spec.graphql.org/draft/#OperationType
    OPERATION_TYPE,

    /// https://spec.graphql.org/draft/#DirectiveLocations
    DIRECTIVE_LOCATIONS,

    /// https://spec.graphql.org/draft/#DirectiveLocation
    DIRECTIVE_LOCATION,

    /// https://spec.graphql.org/draft/#ExecutableDirectiveLocation
    EXECUTABLE_DIRECTIVE_LOCATION,

    /// https://spec.graphql.org/draft/#TypeSystemDirectiveLocation
    TYPE_SYSTEM_DIRECTIVE_LOCATION,

    /// https://spec.graphql.org/draft/#Description
    DESCRIPTION,

    /// https://spec.graphql.org/draft/#Variable
    VARIABLE,

    /// https://spec.graphql.org/draft/#DefaultValue
    DEFAULT_VALUE,

    /// https://spec.graphql.org/draft/#Directives
    DIRECTIVES,

    /// https://spec.graphql.org/draft/#Directive
    DIRECTIVE,

    /// https://spec.graphql.org/draft/#Arguments
    ARGUMENTS,

    /// https://spec.graphql.org/draft/#Argument
    ARGUMENT,

    /// https://spec.graphql.org/draft/#Type
    TYPE,

    /// https://spec.graphql.org/draft/#NamedType
    NAMED_TYPE,

    /// https://spec.graphql.org/draft/#ListType
    LIST_TYPE,

    /// https://spec.graphql.org/draft/#NonNullType
    NON_NULL_TYPE,

    /// https://spec.graphql.org/draft/#SelectionSet
    SELECTION_SET,

    /// https://spec.graphql.org/draft/#Selection
    SELECTION,

    /// https://spec.graphql.org/draft/#Field
    FIELD,

    /// https://spec.graphql.org/draft/#Alias
    ALIAS,

    /// https://spec.graphql.org/draft/#FragmentSpread
    FRAGMENT_SPREAD,

    /// https://spec.graphql.org/draft/#FragmentName
    FRAGMENT_NAME,

    /// https://spec.graphql.org/draft/#sec-Type-Conditions
    TYPE_CONDITION,

    /// https://spec.graphql.org/draft/#sec-Inline-Fragments
    INLINE_FRAGMENT,

    /// https://spec.graphql.org/draft/#Value
    VALUE,

    /// https://spec.graphql.org/draft/#IntValue
    INT_VALUE,

    /// https://spec.graphql.org/draft/#FloatValue
    FLOAT_VALUE,

    /// https://spec.graphql.org/draft/#StringValue
    STRING_VALUE,

    /// https://spec.graphql.org/draft/#BooleanValue
    BOOLEAN_VALUE,

    /// https://spec.graphql.org/draft/#NullValue
    NULL_VALUE,

    /// https://spec.graphql.org/draft/#EnumValue
    ENUM_VALUE,

    /// https://spec.graphql.org/draft/#ListValue
    LIST_VALUE,

    /// https://spec.graphql.org/draft/#ObjectValue
    OBJECT_VALUE,

    /// https://spec.graphql.org/draft/#ObjectField
    OBJECT_FIELD,

    /// https://spec.graphql.org/draft/#BlockString
    BLOCK_STRING,

    /// https://spec.graphql.org/draft/#IntegerPart
    INTEGER_PART,

    /// https://spec.graphql.org/draft/#NegativeSign
    NEGATIVE_SIGN,

    /// https://spec.graphql.org/draft/#NonZeroDigit
    NON_ZERO_DIGIT,

    /// https://spec.graphql.org/draft/#ExponentIndicator
    EXPONENT_INDICATOR,

    /// https://spec.graphql.org/draft/#FractionalPart
    FRACTIONAL_PART,

    /// https://spec.graphql.org/draft/#ExponentPart
    EXPONENT_PART,

    /// https://spec.graphql.org/draft/#Sign
    SIGN,

    /// https://spec.graphql.org/draft/#sec-White-Space
    WHITE_SPACE,

    /// https://spec.graphql.org/draft/#sec-Line-Terminators
    LINE_TERMINATOR,

    /// https://spec.graphql.org/draft/#sec-Comments
    COMMENT,

    /// https://spec.graphql.org/draft/#sec-Insignificant-Commas
    INSIGNIFICANT_COMMA,

    /// https://spec.graphql.org/draft/#sec-Language.Source-Text.Ignored-Tokens
    IGNORED_TOKEN,

    /// https://spec.graphql.org/draft/#Punctuator
    PUNCTUATOR,

    /// https://spec.graphql.org/draft/#sec-Names
    NAME,

    KEYWORD_FRAGMENT,     // fragment
    KEYWORD_QUERY,        // query
    KEYWORD_MUTATION,     // mutation
    KEYWORD_SUBSCRIPTION, // subscription
    KEYWORD_TRUE,         // true
    KEYWORD_FALSE,        // false
    KEYWORD_NULL,         // null
    KEYWORD_REPEATABLE,   // repeatable
    KEYWORD_ON,           // on
    KEYWORD_SCALAR,       // scalar
    KEYWORD_SCHEMA,       // schema
    KEYWORD_TYPE,         // type
    KEYWORD_INTERFACE,    // interface
    KEYWORD_UNION,        // union
    KEYWORD_ENUM,         // enum
    KEYWORD_INPUT,        // input
    KEYWORD_DIRECTIVE,    // directive
    KEYWORD_IMPLEMENTS,   // implements
    KEYWORD_EXTEND,       // extend
    KEYWORD_EXTENDS,      // extends

    SYMBOL_LEFT_PARENS,   // (
    SYMBOL_RIGHT_PARENS,  // )
    SYMBOL_COLON,         // :
    SYMBOL_EQUAL,         // =
    SYMBOL_AT,            // @
    SYMBOL_DOLLAR,        // $
    SYMBOL_AMPERSAND,     // &
    SYMBOL_LEFT_BRACKET,  // [
    SYMBOL_RIGHT_BRACKET, // ]
    SYMBOL_LEFT_BRACE,    // {
    SYMBOL_RIGHT_BRACE,   // }
    SYMBOL_EXCLAMATION,   // !
    SYMBOL_PIPE,          // |
    SYMBOL_SPREAD,        // ...

    // https://spec.graphql.org/draft/#sec-Location
    LOCATION_QUERY,               // QUERY
    LOCATION_MUTATION,            // MUTATION
    LOCATION_SUBSCRIPTION,        // SUBSCRIPTION
    LOCATION_FIELD,               // FIELD
    LOCATION_FRAGMENT_DEFINITION, // FRAGMENT_DEFINITION
    LOCATION_FRAGMENT_SPREAD,     // FRAGMENT_SPREAD
    LOCATION_INLINE_FRAGMENT,     // INLINE_FRAGMENT
    LOCATION_VARIABLE_DEFINITION, // VARIABLE_DEFINITION

    // https://spec.graphql.org/draft/#sec-Location
    LOCATION_SCHEMA,                 // SCHEMA
    LOCATION_SCALAR,                 // SCALAR
    LOCATION_OBJECT,                 // OBJECT
    LOCATION_FIELD_DEFINITION,       // FIELD_DEFINITION
    LOCATION_ARGUMENT_DEFINITION,    // ARGUMENT_DEFINITION
    LOCATION_INTERFACE,              // INTERFACE
    LOCATION_UNION,                  // UNION
    LOCATION_ENUM,                   // ENUM
    LOCATION_ENUM_VALUE,             // ENUM_VALUE
    LOCATION_INPUT_OBJECT,           // INPUT_OBJECT
    LOCATION_INPUT_FIELD_DEFINITION, // INPUT_FIELD_DEFINITION
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl rowan::Language for SyntaxKind {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
