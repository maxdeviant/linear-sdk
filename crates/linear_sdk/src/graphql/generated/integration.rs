#![allow(clippy::all, warnings)]
pub struct Integration;
pub mod integration {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Integration";
    pub const QUERY : & str = "query Integration($id: String!) {\n    integration(id: $id) {\n        ...Integration\n    }\n}\n\nfragment Integration on Integration {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    service\n}" ;
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
    type DateTime = crate::graphql::custom_scalars::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Integration {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub service: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub integration: IntegrationIntegration,
    }
    pub type IntegrationIntegration = Integration;
}
impl graphql_client::GraphQLQuery for Integration {
    type Variables = integration::Variables;
    type ResponseData = integration::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: integration::QUERY,
            operation_name: integration::OPERATION_NAME,
        }
    }
}
