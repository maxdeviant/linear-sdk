#![allow(clippy::all, warnings)]
pub struct Milestone;
pub mod milestone {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Milestone";
    pub const QUERY : & str = "query Milestone($id: String!) {\n    milestone(id: $id) {\n        ...Milestone\n    }\n}\n\nfragment Milestone on Milestone {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    sortOrder\n    description\n    targetDate\n}" ;
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
    type TimelessDate = crate::graphql::custom_scalars::TimelessDate;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Milestone {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        #[serde(rename = "sortOrder")]
        pub sort_order: Float,
        pub description: Option<String>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<TimelessDate>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[deprecated(note = "Milestones will be removed. Use roadmaps instead.")]
        pub milestone: MilestoneMilestone,
    }
    pub type MilestoneMilestone = Milestone;
}
impl graphql_client::GraphQLQuery for Milestone {
    type Variables = milestone::Variables;
    type ResponseData = milestone::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: milestone::QUERY,
            operation_name: milestone::OPERATION_NAME,
        }
    }
}
