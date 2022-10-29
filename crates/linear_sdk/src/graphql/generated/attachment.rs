#![allow(clippy::all, warnings)]
pub struct Attachment;
pub mod attachment {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Attachment";
    pub const QUERY : & str = "query Attachment($id: String!) {\n    attachment(id: $id) {\n        ...Attachment\n    }\n}\n\nfragment Attachment on Attachment {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    title\n    subtitle\n    url\n    metadata\n    source\n    sourceType\n    groupBySource\n}" ;
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
    pub struct Attachment {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub title: String,
        pub subtitle: Option<String>,
        pub url: String,
        pub metadata: JSONObject,
        pub source: Option<JSONObject>,
        #[serde(rename = "sourceType")]
        pub source_type: Option<JSONObject>,
        #[serde(rename = "groupBySource")]
        pub group_by_source: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub attachment: AttachmentAttachment,
    }
    pub type AttachmentAttachment = Attachment;
}
impl graphql_client::GraphQLQuery for Attachment {
    type Variables = attachment::Variables;
    type ResponseData = attachment::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: attachment::QUERY,
            operation_name: attachment::OPERATION_NAME,
        }
    }
}
