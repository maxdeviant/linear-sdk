#![allow(clippy::all, warnings)]
pub struct Integrations;
pub mod integrations {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Integrations";
    pub const QUERY : & str = "query Integrations($before: String, $after: String, $first: Int, $last: Int, $include_archived: Boolean, $order_by: PaginationOrderBy) {\n    integrations(before: $before, after: $after, first: $first, last: $last, includeArchived: $include_archived, orderBy: $order_by) {\n        ...IntegrationConnection\n    }\n}\n\nfragment IntegrationConnection on IntegrationConnection {\n    __typename\n    \n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Debug)]
    pub enum PaginationOrderBy {
        createdAt,
        updatedAt,
        Other(String),
    }
    impl ::serde::Serialize for PaginationOrderBy {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                PaginationOrderBy::createdAt => "createdAt",
                PaginationOrderBy::updatedAt => "updatedAt",
                PaginationOrderBy::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PaginationOrderBy {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "createdAt" => Ok(PaginationOrderBy::createdAt),
                "updatedAt" => Ok(PaginationOrderBy::updatedAt),
                _ => Ok(PaginationOrderBy::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub before: Option<String>,
        pub after: Option<String>,
        pub first: Option<Int>,
        pub last: Option<Int>,
        pub include_archived: Option<Boolean>,
        pub order_by: Option<PaginationOrderBy>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum IntegrationConnection {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub integrations: IntegrationsIntegrations,
    }
    pub type IntegrationsIntegrations = IntegrationConnection;
}
impl graphql_client::GraphQLQuery for Integrations {
    type Variables = integrations::Variables;
    type ResponseData = integrations::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: integrations::QUERY,
            operation_name: integrations::OPERATION_NAME,
        }
    }
}
