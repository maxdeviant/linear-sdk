#![allow(clippy::all, warnings)]
pub struct ArchivedModelsSync;
pub mod archived_models_sync {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ArchivedModelsSync";
    pub const QUERY : & str = "query ArchivedModelsSync($model_class: String!, $team_id: String!, $trash_option: TrashOptionType, $before: DateTime, $before_id: String, $last: Int) {\n    archivedModelsSync(modelClass: $model_class, teamId: $team_id, trashOption: $trash_option, before: $before, beforeId: $before_id, last: $last) {\n        ...ArchiveResponse\n    }\n}\n\nfragment ArchiveResponse on ArchiveResponse {\n    __typename\n    archive\n    totalCount\n    databaseVersion\n    includesDependencies\n}" ;
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
    #[derive(Debug)]
    pub enum TrashOptionType {
        includeTrash,
        excludeTrash,
        trashOnly,
        Other(String),
    }
    impl ::serde::Serialize for TrashOptionType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TrashOptionType::includeTrash => "includeTrash",
                TrashOptionType::excludeTrash => "excludeTrash",
                TrashOptionType::trashOnly => "trashOnly",
                TrashOptionType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrashOptionType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "includeTrash" => Ok(TrashOptionType::includeTrash),
                "excludeTrash" => Ok(TrashOptionType::excludeTrash),
                "trashOnly" => Ok(TrashOptionType::trashOnly),
                _ => Ok(TrashOptionType::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub model_class: String,
        pub team_id: String,
        pub trash_option: Option<TrashOptionType>,
        pub before: Option<DateTime>,
        pub before_id: Option<String>,
        pub last: Option<Int>,
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
        #[serde(rename = "archivedModelsSync")]
        pub archived_models_sync: ArchivedModelsSyncArchivedModelsSync,
    }
    pub type ArchivedModelsSyncArchivedModelsSync = ArchiveResponse;
}
impl graphql_client::GraphQLQuery for ArchivedModelsSync {
    type Variables = archived_models_sync::Variables;
    type ResponseData = archived_models_sync::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: archived_models_sync::QUERY,
            operation_name: archived_models_sync::OPERATION_NAME,
        }
    }
}
