#![allow(clippy::all, warnings)]
pub struct IntegrationTemplate;
pub mod integration_template {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IntegrationTemplate";
    pub const QUERY : & str = "query IntegrationTemplate($id: String!) {\n    integrationTemplate(id: $id) {\n        ...IntegrationTemplate\n    }\n}\n\nfragment IntegrationTemplate on IntegrationTemplate {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n}" ;
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
    pub struct IntegrationTemplate {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "integrationTemplate")]
        pub integration_template: IntegrationTemplateIntegrationTemplate,
    }
    pub type IntegrationTemplateIntegrationTemplate = IntegrationTemplate;
}
impl graphql_client::GraphQLQuery for IntegrationTemplate {
    type Variables = integration_template::Variables;
    type ResponseData = integration_template::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: integration_template::QUERY,
            operation_name: integration_template::OPERATION_NAME,
        }
    }
}
