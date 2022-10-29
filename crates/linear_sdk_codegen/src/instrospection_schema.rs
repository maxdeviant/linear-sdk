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
    pub types: Vec<GraphQlType>,
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
pub struct GraphQlType {
    pub kind: GraphQlTypeKind,
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Vec<Field>>,
    pub of_type: Option<Box<GraphQlType>>,
}

#[derive(Debug, Deserialize)]
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
    pub ty: GraphQlType,

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
    pub ty: GraphQlType,

    pub default_value: Option<String>,
}
