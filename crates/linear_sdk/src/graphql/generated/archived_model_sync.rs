#![allow(clippy::all, warnings)]
pub struct ArchivedModelSync;
pub mod archived_model_sync {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ArchivedModelSync";
    pub const QUERY : & str = "query ArchivedModelSync($identifier: String!, $model_class: String!) {\n    archivedModelSync(identifier: $identifier, modelClass: $model_class) {\n        ...ArchiveResponse\n    }\n}\n\nfragment ArchiveResponse on ArchiveResponse {\n    __typename\n    archive\n    totalCount\n    databaseVersion\n    includesDependencies\n}" ;
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
        pub identifier: String,
        pub model_class: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ArchiveResponse {
        pub archive: String,
        #[serde(rename = "totalCount")]
        pub total_count: Float,
        #[serde(rename = "databaseVersion")]
        pub database_version: Float,
        #[serde(rename = "includesDependencies")]
        pub includes_dependencies: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "archivedModelSync")]
        pub archived_model_sync: ArchivedModelSyncArchivedModelSync,
    }
    pub type ArchivedModelSyncArchivedModelSync = ArchiveResponse;
}
impl graphql_client::GraphQLQuery for ArchivedModelSync {
    type Variables = archived_model_sync::Variables;
    type ResponseData = archived_model_sync::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: archived_model_sync::QUERY,
            operation_name: archived_model_sync::OPERATION_NAME,
        }
    }
}
