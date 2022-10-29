#![allow(clippy::all, warnings)]
pub struct Roadmap;
pub mod roadmap {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Roadmap";
    pub const QUERY : & str = "query Roadmap($id: String!) {\n    roadmap(id: $id) {\n        ...Roadmap\n    }\n}\n\nfragment Roadmap on Roadmap {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    description\n    slugId\n}" ;
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
    pub struct Roadmap {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        pub description: Option<String>,
        #[serde(rename = "slugId")]
        pub slug_id: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub roadmap: RoadmapRoadmap,
    }
    pub type RoadmapRoadmap = Roadmap;
}
impl graphql_client::GraphQLQuery for Roadmap {
    type Variables = roadmap::Variables;
    type ResponseData = roadmap::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: roadmap::QUERY,
            operation_name: roadmap::OPERATION_NAME,
        }
    }
}
