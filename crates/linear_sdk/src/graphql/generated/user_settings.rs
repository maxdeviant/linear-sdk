#![allow(clippy::all, warnings)]
pub struct UserSettings;
pub mod user_settings {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UserSettings";
    pub const QUERY : & str = "query UserSettings {\n    userSettings {\n        ...UserSettings\n    }\n}\n\nfragment UserSettings on UserSettings {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    notificationPreferences\n    unsubscribedFrom\n}" ;
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
    type JSONObject = crate::graphql::custom_scalars::JSONObject;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct UserSettings {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "notificationPreferences")]
        pub notification_preferences: JSONObject,
        #[serde(rename = "unsubscribedFrom")]
        pub unsubscribed_from: Vec<String>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "userSettings")]
        pub user_settings: UserSettingsUserSettings,
    }
    pub type UserSettingsUserSettings = UserSettings;
}
impl graphql_client::GraphQLQuery for UserSettings {
    type Variables = user_settings::Variables;
    type ResponseData = user_settings::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_settings::QUERY,
            operation_name: user_settings::OPERATION_NAME,
        }
    }
}
