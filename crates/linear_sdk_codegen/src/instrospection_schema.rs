use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IntrospectionQuery {
    pub data: IntrospectionQueryData,
}

#[derive(Debug, Deserialize)]
pub struct IntrospectionQueryData {
    #[serde(rename = "__schema")]
    pub schema: IntrospectionSchema,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntrospectionSchema {
    pub query_type: QueryType,
    pub mutation_type: Option<MutationType>,
    pub types: Vec<GraphQlFullType>,
}

#[derive(Debug, Deserialize)]
pub struct QueryType {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct MutationType {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlFullType {
    pub kind: GraphQlTypeKind,
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Vec<Field>>,
    pub of_type: Option<GraphQlTypeRef>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GraphQlTypeKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub description: Option<String>,

    #[serde(rename = "type")]
    pub ty: GraphQlTypeRef,

    pub args: Vec<InputValue>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputValue {
    pub name: String,
    pub description: Option<String>,

    #[serde(rename = "type")]
    pub ty: GraphQlTypeRef,

    pub default_value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GraphQlTypeRef {
    Scalar { name: String },
    Object { name: String },
    Interface { name: String },
    Union { name: String },
    Enum { name: String },
    InputObject { name: String },
    NonNull(Box<OfType>),
    List(Box<OfType>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfType {
    pub of_type: GraphQlTypeRef,
}
