#![allow(clippy::all, warnings)]
pub struct Document;
pub mod document {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Document";
    pub const QUERY : & str = "query Document($id: String!) {\n    document(id: $id) {\n        ...Document\n    }\n}\n\nfragment Document on Document {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    title\n    content\n    contentData\n    icon\n    color\n    slugId\n}" ;
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
    type JSONObject = crate::graphql::custom_scalars::JSONObject;
    type DateTime = crate::graphql::custom_scalars::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Document {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub title: String,
        pub content: Option<String>,
        #[serde(rename = "contentData")]
        pub content_data: Option<JSONObject>,
        pub icon: Option<String>,
        pub color: Option<String>,
        #[serde(rename = "slugId")]
        pub slug_id: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub document: DocumentDocument,
    }
    pub type DocumentDocument = Document;
}
impl graphql_client::GraphQLQuery for Document {
    type Variables = document::Variables;
    type ResponseData = document::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: document::QUERY,
            operation_name: document::OPERATION_NAME,
        }
    }
}
