#![allow(clippy::all, warnings)]
pub struct IntegrationResource;
pub mod integration_resource {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IntegrationResource";
    pub const QUERY : & str = "query IntegrationResource($id: String!) {\n    integrationResource(id: $id) {\n        ...IntegrationResource\n    }\n}\n\nfragment IntegrationResource on IntegrationResource {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    resourceType\n    resourceId\n}" ;
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
    pub struct IntegrationResource {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "resourceType")]
        pub resource_type: String,
        #[serde(rename = "resourceId")]
        pub resource_id: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "integrationResource")]
        #[deprecated(note = "This query will soon be deprecated, please use `attachment` instead")]
        pub integration_resource: IntegrationResourceIntegrationResource,
    }
    pub type IntegrationResourceIntegrationResource = IntegrationResource;
}
impl graphql_client::GraphQLQuery for IntegrationResource {
    type Variables = integration_resource::Variables;
    type ResponseData = integration_resource::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: integration_resource::QUERY,
            operation_name: integration_resource::OPERATION_NAME,
        }
    }
}
