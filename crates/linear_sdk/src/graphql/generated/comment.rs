#![allow(clippy::all, warnings)]
pub struct Comment;
pub mod comment {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Comment";
    pub const QUERY : & str = "query Comment($id: String!) {\n    comment(id: $id) {\n        ...Comment\n    }\n}\n\nfragment Comment on Comment {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    body\n    editedAt\n    bodyData\n    url\n}" ;
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
    pub struct Comment {
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
        #[serde(rename = "bodyData")]
        pub body_data: String,
        pub url: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub comment: CommentComment,
    }
    pub type CommentComment = Comment;
}
impl graphql_client::GraphQLQuery for Comment {
    type Variables = comment::Variables;
    type ResponseData = comment::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: comment::QUERY,
            operation_name: comment::OPERATION_NAME,
        }
    }
}
