#![allow(clippy::all, warnings)]
pub struct RoadmapToProject;
pub mod roadmap_to_project {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RoadmapToProject";
    pub const QUERY : & str = "query RoadmapToProject($id: String!) {\n    roadmapToProject(id: $id) {\n        ...RoadmapToProject\n    }\n}\n\nfragment RoadmapToProject on RoadmapToProject {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    sortOrder\n}" ;
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
    pub struct RoadmapToProject {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "sortOrder")]
        pub sort_order: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "roadmapToProject")]
        pub roadmap_to_project: RoadmapToProjectRoadmapToProject,
    }
    pub type RoadmapToProjectRoadmapToProject = RoadmapToProject;
}
impl graphql_client::GraphQLQuery for RoadmapToProject {
    type Variables = roadmap_to_project::Variables;
    type ResponseData = roadmap_to_project::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: roadmap_to_project::QUERY,
            operation_name: roadmap_to_project::OPERATION_NAME,
        }
    }
}
