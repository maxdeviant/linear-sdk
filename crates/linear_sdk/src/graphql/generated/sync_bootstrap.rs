#![allow(clippy::all, warnings)]
pub struct SyncBootstrap;
pub mod sync_bootstrap {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SyncBootstrap";
    pub const QUERY : & str = "query SyncBootstrap($only_models: [String!], $sync_groups: [String!]) {\n    syncBootstrap(onlyModels: $only_models, syncGroups: $sync_groups) {\n        ...SyncResponse\n    }\n}\n\nfragment SyncResponse on SyncResponse {\n    __typename\n    state\n    delta\n    subscribedSyncGroups\n    lastSyncId\n    databaseVersion\n}" ;
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
    #[derive(Serialize)]
    pub struct Variables {
        pub only_models: Option<Vec<String>>,
        pub sync_groups: Option<Vec<String>>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct SyncResponse {
        pub state: Option<String>,
        pub delta: Option<String>,
        #[serde(rename = "subscribedSyncGroups")]
        pub subscribed_sync_groups: Vec<String>,
        #[serde(rename = "lastSyncId")]
        pub last_sync_id: Float,
        #[serde(rename = "databaseVersion")]
        pub database_version: Float,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "syncBootstrap")]
        pub sync_bootstrap: SyncBootstrapSyncBootstrap,
    }
    pub type SyncBootstrapSyncBootstrap = SyncResponse;
}
impl graphql_client::GraphQLQuery for SyncBootstrap {
    type Variables = sync_bootstrap::Variables;
    type ResponseData = sync_bootstrap::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: sync_bootstrap::QUERY,
            operation_name: sync_bootstrap::OPERATION_NAME,
        }
    }
}
