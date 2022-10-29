#![allow(clippy::all, warnings)]
pub struct SyncEntityCount;
pub mod sync_entity_count {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SyncEntityCount";
    pub const QUERY : & str = "query SyncEntityCount {\n    syncEntityCount {\n        ...EntityCountResponse\n    }\n}\n\nfragment EntityCountResponse on EntityCountResponse {\n    __typename\n    counts\n}" ;
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
    type JSON = crate::graphql::custom_scalars::JSON;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct EntityCountResponse {
        pub counts: JSON,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "syncEntityCount")]
        pub sync_entity_count: SyncEntityCountSyncEntityCount,
    }
    pub type SyncEntityCountSyncEntityCount = EntityCountResponse;
}
impl graphql_client::GraphQLQuery for SyncEntityCount {
    type Variables = sync_entity_count::Variables;
    type ResponseData = sync_entity_count::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: sync_entity_count::QUERY,
            operation_name: sync_entity_count::OPERATION_NAME,
        }
    }
}
