#![allow(clippy::all, warnings)]
pub struct ProjectUpdateInteraction;
pub mod project_update_interaction {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProjectUpdateInteraction";
    pub const QUERY : & str = "query ProjectUpdateInteraction($id: String!) {\n    projectUpdateInteraction(id: $id) {\n        ...ProjectUpdateInteraction\n    }\n}\n\nfragment ProjectUpdateInteraction on ProjectUpdateInteraction {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    readAt\n}" ;
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
    pub struct ProjectUpdateInteraction {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "readAt")]
        pub read_at: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "projectUpdateInteraction")]
        pub project_update_interaction: ProjectUpdateInteractionProjectUpdateInteraction,
    }
    pub type ProjectUpdateInteractionProjectUpdateInteraction = ProjectUpdateInteraction;
}
impl graphql_client::GraphQLQuery for ProjectUpdateInteraction {
    type Variables = project_update_interaction::Variables;
    type ResponseData = project_update_interaction::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_update_interaction::QUERY,
            operation_name: project_update_interaction::OPERATION_NAME,
        }
    }
}
