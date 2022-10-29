#![allow(clippy::all, warnings)]
pub struct Cycle;
pub mod cycle {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Cycle";
    pub const QUERY : & str = "query Cycle($id: String!) {\n    cycle(id: $id) {\n        ...Cycle\n    }\n}\n\nfragment Cycle on Cycle {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    number\n    name\n    description\n    startsAt\n    endsAt\n    completedAt\n    autoArchivedAt\n    issueCountHistory\n    completedIssueCountHistory\n    scopeHistory\n    completedScopeHistory\n    inProgressScopeHistory\n    progress\n}" ;
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
    pub struct Cycle {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub number: Float,
        pub name: Option<String>,
        pub description: Option<String>,
        #[serde(rename = "startsAt")]
        pub starts_at: DateTime,
        #[serde(rename = "endsAt")]
        pub ends_at: DateTime,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateTime>,
        #[serde(rename = "autoArchivedAt")]
        pub auto_archived_at: Option<DateTime>,
        #[serde(rename = "issueCountHistory")]
        pub issue_count_history: Vec<Float>,
        #[serde(rename = "completedIssueCountHistory")]
        pub completed_issue_count_history: Vec<Float>,
        #[serde(rename = "scopeHistory")]
        pub scope_history: Vec<Float>,
        #[serde(rename = "completedScopeHistory")]
        pub completed_scope_history: Vec<Float>,
        #[serde(rename = "inProgressScopeHistory")]
        pub in_progress_scope_history: Vec<Float>,
        pub progress: Float,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub cycle: CycleCycle,
    }
    pub type CycleCycle = Cycle;
}
impl graphql_client::GraphQLQuery for Cycle {
    type Variables = cycle::Variables;
    type ResponseData = cycle::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: cycle::QUERY,
            operation_name: cycle::OPERATION_NAME,
        }
    }
}
