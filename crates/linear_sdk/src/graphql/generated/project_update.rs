#![allow(clippy::all, warnings)]
pub struct ProjectUpdate;
pub mod project_update {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProjectUpdate";
    pub const QUERY : & str = "query ProjectUpdate($id: String!) {\n    projectUpdate(id: $id) {\n        ...ProjectUpdate\n    }\n}\n\nfragment ProjectUpdate on ProjectUpdate {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    body\n    editedAt\n    url\n}" ;
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
    pub struct ProjectUpdate {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub body: String,
        #[serde(rename = "editedAt")]
        pub edited_at: Option<DateTime>,
        pub url: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "projectUpdate")]
        pub project_update: ProjectUpdateProjectUpdate,
    }
    pub type ProjectUpdateProjectUpdate = ProjectUpdate;
}
impl graphql_client::GraphQLQuery for ProjectUpdate {
    type Variables = project_update::Variables;
    type ResponseData = project_update::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_update::QUERY,
            operation_name: project_update::OPERATION_NAME,
        }
    }
}
