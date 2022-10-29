#![allow(clippy::all, warnings)]
pub struct DependentModelSync;
pub mod dependent_model_sync {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DependentModelSync";
    pub const QUERY : & str = "query DependentModelSync($include_dependent: Boolean, $identifier: String!, $model_class: String!) {\n    dependentModelSync(includeDependent: $include_dependent, identifier: $identifier, modelClass: $model_class) {\n        ...DependencyResponse\n    }\n}\n\nfragment DependencyResponse on DependencyResponse {\n    __typename\n    dependencies\n}" ;
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
        pub include_dependent: Option<Boolean>,
        pub identifier: String,
        pub model_class: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct DependencyResponse {
        pub dependencies: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "dependentModelSync")]
        pub dependent_model_sync: DependentModelSyncDependentModelSync,
    }
    pub type DependentModelSyncDependentModelSync = DependencyResponse;
}
impl graphql_client::GraphQLQuery for DependentModelSync {
    type Variables = dependent_model_sync::Variables;
    type ResponseData = dependent_model_sync::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: dependent_model_sync::QUERY,
            operation_name: dependent_model_sync::OPERATION_NAME,
        }
    }
}
