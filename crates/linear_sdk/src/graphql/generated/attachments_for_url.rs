#![allow(clippy::all, warnings)]
pub struct AttachmentsForUrl;
pub mod attachments_for_url {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "AttachmentsForUrl";
    pub const QUERY : & str = "query AttachmentsForUrl($before: String, $after: String, $first: Int, $last: Int, $include_archived: Boolean, $order_by: PaginationOrderBy, $url: String!) {\n    attachmentsForURL(before: $before, after: $after, first: $first, last: $last, includeArchived: $include_archived, orderBy: $order_by, url: $url) {\n        ...AttachmentConnection\n    }\n}\n\nfragment AttachmentConnection on AttachmentConnection {\n    __typename\n    \n}" ;
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
    #[derive(Debug)]
    pub enum PaginationOrderBy {
        createdAt,
        updatedAt,
        Other(String),
    }
    impl ::serde::Serialize for PaginationOrderBy {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                PaginationOrderBy::createdAt => "createdAt",
                PaginationOrderBy::updatedAt => "updatedAt",
                PaginationOrderBy::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PaginationOrderBy {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "createdAt" => Ok(PaginationOrderBy::createdAt),
                "updatedAt" => Ok(PaginationOrderBy::updatedAt),
                _ => Ok(PaginationOrderBy::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub before: Option<String>,
        pub after: Option<String>,
        pub first: Option<Int>,
        pub last: Option<Int>,
        pub include_archived: Option<Boolean>,
        pub order_by: Option<PaginationOrderBy>,
        pub url: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum AttachmentConnection {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "attachmentsForURL")]
        pub attachments_for_url: AttachmentsForUrlAttachmentsForUrl,
    }
    pub type AttachmentsForUrlAttachmentsForUrl = AttachmentConnection;
}
impl graphql_client::GraphQLQuery for AttachmentsForUrl {
    type Variables = attachments_for_url::Variables;
    type ResponseData = attachments_for_url::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: attachments_for_url::QUERY,
            operation_name: attachments_for_url::OPERATION_NAME,
        }
    }
}
