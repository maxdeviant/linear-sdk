#![allow(clippy::all, warnings)]
pub struct Organization;
pub mod organization {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Organization";
    pub const QUERY : & str = "query Organization {\n    organization {\n        ...Organization\n    }\n}\n\nfragment Organization on Organization {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    urlKey\n    logoUrl\n    periodUploadVolume\n    gitBranchFormat\n    gitLinkbackMessagesEnabled\n    gitPublicLinkbackMessagesEnabled\n    roadmapEnabled\n    projectUpdateRemindersHour\n    samlEnabled\n    scimEnabled\n    allowedAuthServices\n    deletionRequestedAt\n    userCount\n    createdIssueCount\n}" ;
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
    pub struct Organization {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        #[serde(rename = "urlKey")]
        pub url_key: String,
        #[serde(rename = "logoUrl")]
        pub logo_url: Option<String>,
        #[serde(rename = "periodUploadVolume")]
        pub period_upload_volume: Float,
        #[serde(rename = "gitBranchFormat")]
        pub git_branch_format: Option<String>,
        #[serde(rename = "gitLinkbackMessagesEnabled")]
        pub git_linkback_messages_enabled: Boolean,
        #[serde(rename = "gitPublicLinkbackMessagesEnabled")]
        pub git_public_linkback_messages_enabled: Boolean,
        #[serde(rename = "roadmapEnabled")]
        pub roadmap_enabled: Boolean,
        #[serde(rename = "projectUpdateRemindersHour")]
        pub project_update_reminders_hour: Float,
        #[serde(rename = "samlEnabled")]
        pub saml_enabled: Boolean,
        #[serde(rename = "scimEnabled")]
        pub scim_enabled: Boolean,
        #[serde(rename = "allowedAuthServices")]
        pub allowed_auth_services: Vec<String>,
        #[serde(rename = "deletionRequestedAt")]
        pub deletion_requested_at: Option<DateTime>,
        #[serde(rename = "userCount")]
        pub user_count: Int,
        #[serde(rename = "createdIssueCount")]
        pub created_issue_count: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub organization: OrganizationOrganization,
    }
    pub type OrganizationOrganization = Organization;
}
impl graphql_client::GraphQLQuery for Organization {
    type Variables = organization::Variables;
    type ResponseData = organization::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: organization::QUERY,
            operation_name: organization::OPERATION_NAME,
        }
    }
}
