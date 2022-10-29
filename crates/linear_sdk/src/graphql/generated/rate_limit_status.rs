#![allow(clippy::all, warnings)]
pub struct RateLimitStatus;
pub mod rate_limit_status {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RateLimitStatus";
    pub const QUERY : & str = "query RateLimitStatus {\n    rateLimitStatus {\n        ...RateLimitPayload\n    }\n}\n\nfragment RateLimitPayload on RateLimitPayload {\n    __typename\n    identifier\n    kind\n}" ;
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
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct RateLimitPayload {
        pub identifier: Option<String>,
        pub kind: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "rateLimitStatus")]
        pub rate_limit_status: RateLimitStatusRateLimitStatus,
    }
    pub type RateLimitStatusRateLimitStatus = RateLimitPayload;
}
impl graphql_client::GraphQLQuery for RateLimitStatus {
    type Variables = rate_limit_status::Variables;
    type ResponseData = rate_limit_status::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: rate_limit_status::QUERY,
            operation_name: rate_limit_status::OPERATION_NAME,
        }
    }
}
