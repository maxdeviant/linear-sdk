#![allow(clippy::all, warnings)]
pub struct ProjectLink;
pub mod project_link {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProjectLink";
    pub const QUERY : & str = "query ProjectLink($id: String!) {\n    projectLink(id: $id) {\n        ...ProjectLink\n    }\n}\n\nfragment ProjectLink on ProjectLink {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    url\n    label\n}" ;
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
    pub struct ProjectLink {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub url: String,
        pub label: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "projectLink")]
        pub project_link: ProjectLinkProjectLink,
    }
    pub type ProjectLinkProjectLink = ProjectLink;
}
impl graphql_client::GraphQLQuery for ProjectLink {
    type Variables = project_link::Variables;
    type ResponseData = project_link::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_link::QUERY,
            operation_name: project_link::OPERATION_NAME,
        }
    }
}
