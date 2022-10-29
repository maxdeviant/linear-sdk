#![allow(clippy::all, warnings)]
pub struct Webhook;
pub mod webhook {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Webhook";
    pub const QUERY : & str = "query Webhook($id: String!) {\n    webhook(id: $id) {\n        ...Webhook\n    }\n}\n\nfragment Webhook on Webhook {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    label\n    url\n    enabled\n    allPublicTeams\n    secret\n    resourceTypes\n}" ;
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
    pub struct Webhook {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub label: Option<String>,
        pub url: Option<String>,
        pub enabled: Boolean,
        #[serde(rename = "allPublicTeams")]
        pub all_public_teams: Boolean,
        pub secret: Option<String>,
        #[serde(rename = "resourceTypes")]
        pub resource_types: Vec<String>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub webhook: WebhookWebhook,
    }
    pub type WebhookWebhook = Webhook;
}
impl graphql_client::GraphQLQuery for Webhook {
    type Variables = webhook::Variables;
    type ResponseData = webhook::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: webhook::QUERY,
            operation_name: webhook::OPERATION_NAME,
        }
    }
}
