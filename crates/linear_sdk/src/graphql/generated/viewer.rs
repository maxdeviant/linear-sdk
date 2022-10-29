#![allow(clippy::all, warnings)]
pub struct Viewer;
pub mod viewer {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Viewer";
    pub const QUERY : & str = "query Viewer {\n    viewer {\n        ...User\n    }\n}\n\nfragment User on User {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    displayName\n    email\n    avatarUrl\n    disableReason\n    inviteHash\n    calendarHash\n    description\n    statusEmoji\n    statusLabel\n    statusUntilAt\n    timezone\n    lastSeen\n    guest\n    active\n    url\n    createdIssueCount\n    isMe\n    admin\n}" ;
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
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct User {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub email: String,
        #[serde(rename = "avatarUrl")]
        pub avatar_url: Option<String>,
        #[serde(rename = "disableReason")]
        pub disable_reason: Option<String>,
        #[serde(rename = "inviteHash")]
        pub invite_hash: String,
        #[serde(rename = "calendarHash")]
        pub calendar_hash: Option<String>,
        pub description: Option<String>,
        #[serde(rename = "statusEmoji")]
        pub status_emoji: Option<String>,
        #[serde(rename = "statusLabel")]
        pub status_label: Option<String>,
        #[serde(rename = "statusUntilAt")]
        pub status_until_at: Option<DateTime>,
        pub timezone: Option<String>,
        #[serde(rename = "lastSeen")]
        pub last_seen: Option<DateTime>,
        pub guest: Boolean,
        pub active: Boolean,
        pub url: String,
        #[serde(rename = "createdIssueCount")]
        pub created_issue_count: Int,
        #[serde(rename = "isMe")]
        pub is_me: Boolean,
        pub admin: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub viewer: ViewerViewer,
    }
    pub type ViewerViewer = User;
}
impl graphql_client::GraphQLQuery for Viewer {
    type Variables = viewer::Variables;
    type ResponseData = viewer::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: viewer::QUERY,
            operation_name: viewer::OPERATION_NAME,
        }
    }
}
