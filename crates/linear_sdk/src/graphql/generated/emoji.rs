#![allow(clippy::all, warnings)]
pub struct Emoji;
pub mod emoji {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Emoji";
    pub const QUERY : & str = "query Emoji($id: String!) {\n    emoji(id: $id) {\n        ...Emoji\n    }\n}\n\nfragment Emoji on Emoji {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    url\n    source\n}" ;
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
    pub struct Emoji {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        pub url: String,
        pub source: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub emoji: EmojiEmoji,
    }
    pub type EmojiEmoji = Emoji;
}
impl graphql_client::GraphQLQuery for Emoji {
    type Variables = emoji::Variables;
    type ResponseData = emoji::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: emoji::QUERY,
            operation_name: emoji::OPERATION_NAME,
        }
    }
}
