#![allow(clippy::all, warnings)]
pub struct PushSubscriptionTest;
pub mod push_subscription_test {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PushSubscriptionTest";
    pub const QUERY : & str = "query PushSubscriptionTest {\n    pushSubscriptionTest {\n        ...PushSubscriptionTestPayload\n    }\n}\n\nfragment PushSubscriptionTestPayload on PushSubscriptionTestPayload {\n    __typename\n    success\n}" ;
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
    pub struct PushSubscriptionTestPayload {
        pub success: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "pushSubscriptionTest")]
        pub push_subscription_test: PushSubscriptionTestPushSubscriptionTest,
    }
    pub type PushSubscriptionTestPushSubscriptionTest = PushSubscriptionTestPayload;
}
impl graphql_client::GraphQLQuery for PushSubscriptionTest {
    type Variables = push_subscription_test::Variables;
    type ResponseData = push_subscription_test::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: push_subscription_test::QUERY,
            operation_name: push_subscription_test::OPERATION_NAME,
        }
    }
}
