#![allow(clippy::all, warnings)]
pub struct NotificationSubscription;
pub mod notification_subscription {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "NotificationSubscription";
    pub const QUERY : & str = "query NotificationSubscription($id: String!) {\n    notificationSubscription(id: $id) {\n        ...NotificationSubscription\n    }\n}\n\nfragment NotificationSubscription on NotificationSubscription {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    type\n}" ;
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
    pub struct NotificationSubscription {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "type")]
        pub type_: String,
        #[serde(flatten)]
        pub on: NotificationSubscriptionOn,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum NotificationSubscriptionOn {
        ProjectNotificationSubscription,
        TeamNotificationSubscription,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "notificationSubscription")]
        pub notification_subscription: NotificationSubscriptionNotificationSubscription,
    }
    pub type NotificationSubscriptionNotificationSubscription = NotificationSubscription;
}
impl graphql_client::GraphQLQuery for NotificationSubscription {
    type Variables = notification_subscription::Variables;
    type ResponseData = notification_subscription::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: notification_subscription::QUERY,
            operation_name: notification_subscription::OPERATION_NAME,
        }
    }
}
