//! The representation of the GraphQL introspection schema.
//!
//! This is based off of the following files used by `graphql-client`:
//!
//! - [`introspection_schema.graphql`](https://github.com/graphql-rust/graphql-client/blob/0776197ad7cfde2c658490e7c7e627a21ed622cb/graphql_client_cli/src/graphql/introspection_schema.graphql)
//! - [`introspection_query.graphql`](https://github.com/graphql-rust/graphql-client/blob/0776197ad7cfde2c658490e7c7e627a21ed622cb/graphql_client_cli/src/graphql/introspection_query.graphql)

use serde::Deserialize;

/// The response from the GraphQL introspection call.
#[derive(Debug, Deserialize)]
pub struct IntrospectionResponse {
    /// The introspection response data.
    pub data: IntrospectionQuery,
}

/// A GraphQL introspection query.
#[derive(Debug, Deserialize)]
pub struct IntrospectionQuery {
    /// The introspection schema.
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
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GraphQlFullType {
    Scalar(GraphQlScalarType),
    Object(GraphQlObjectType),
    Interface(GraphQlInterfaceType),
    Union,
    Enum(GraphQlEnumType),
    InputObject(GraphQlInputObjectType),
}

impl GraphQlFullType {
    pub fn name(&self) -> Option<String> {
        match self {
            Self::Scalar(scalar) => Some(scalar.name.clone()),
            Self::Object(object) => Some(object.name.clone()),
            Self::Interface(interface) => Some(interface.name.clone()),
            Self::Union => None,
            Self::Enum(r#enum) => Some(r#enum.name.clone()),
            Self::InputObject(input_object) => Some(input_object.name.clone()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlScalarType {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlObjectType {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
    pub of_type: Option<GraphQlTypeRef>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlInterfaceType {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
    pub possible_types: Vec<GraphQlTypeRef>,
}

/// A GraphQL field.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    /// The name of the field.
    pub name: String,

    /// The description of the field.
    pub description: Option<String>,

    /// The type of the field.
    #[serde(rename = "type")]
    pub ty: GraphQlTypeRef,

    /// The arguments to the field.
    pub args: Vec<InputValue>,

    /// Whether the field is deprecated.
    pub is_deprecated: bool,

    /// The reason the field is deprecated.
    pub deprecation_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlEnumType {
    pub name: String,
    pub description: Option<String>,
    pub enum_values: Vec<EnumValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub description: Option<String>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlInputObjectType {
    pub name: String,
    pub description: Option<String>,
    pub input_fields: Vec<InputValue>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputValue {
    pub name: String,
    pub description: Option<String>,

    #[serde(rename = "type")]
    pub ty: GraphQlTypeRef,

    pub default_value: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OfType {
    pub of_type: GraphQlTypeRef,
}
