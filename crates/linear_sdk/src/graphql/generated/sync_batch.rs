#![allow(clippy::all, warnings)]
pub struct SyncBatch;
pub mod sync_batch {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SyncBatch";
    pub const QUERY : & str = "query SyncBatch($requests: [BatchRequest!]!) {\n    syncBatch(requests: $requests) {\n        ...SyncBatchResponse\n    }\n}\n\nfragment SyncBatchResponse on SyncBatchResponse {\n    __typename\n    models\n}" ;
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
    pub struct BatchRequest {
        #[serde(rename = "modelClass")]
        pub model_class: String,
        #[serde(rename = "indexedKey")]
        pub indexed_key: String,
        #[serde(rename = "keyValue")]
        pub key_value: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub requests: Vec<BatchRequest>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct SyncBatchResponse {
        pub models: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "syncBatch")]
        pub sync_batch: SyncBatchSyncBatch,
    }
    pub type SyncBatchSyncBatch = SyncBatchResponse;
}
impl graphql_client::GraphQLQuery for SyncBatch {
    type Variables = sync_batch::Variables;
    type ResponseData = sync_batch::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: sync_batch::QUERY,
            operation_name: sync_batch::OPERATION_NAME,
        }
    }
}
