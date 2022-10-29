#![allow(clippy::all, warnings)]
pub struct Notification;
pub mod notification {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Notification";
    pub const QUERY : & str = "query Notification($id: String!) {\n    notification(id: $id) {\n        ...Notification\n    }\n}\n\nfragment Notification on Notification {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    type\n    readAt\n    emailedAt\n    snoozedUntilAt\n}" ;
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
    pub struct Notification {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "type")]
        pub type_: String,
        #[serde(rename = "readAt")]
        pub read_at: Option<DateTime>,
        #[serde(rename = "emailedAt")]
        pub emailed_at: Option<DateTime>,
        #[serde(rename = "snoozedUntilAt")]
        pub snoozed_until_at: Option<DateTime>,
        #[serde(flatten)]
        pub on: NotificationOn,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum NotificationOn {
        IssueNotification,
        ProjectNotification,
        OauthClientApprovalNotification,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub notification: NotificationNotification,
    }
    pub type NotificationNotification = Notification;
}
impl graphql_client::GraphQLQuery for Notification {
    type Variables = notification::Variables;
    type ResponseData = notification::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: notification::QUERY,
            operation_name: notification::OPERATION_NAME,
        }
    }
}
