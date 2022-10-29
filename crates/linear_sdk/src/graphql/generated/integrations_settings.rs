#![allow(clippy::all, warnings)]
pub struct IntegrationsSettings;
pub mod integrations_settings {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IntegrationsSettings";
    pub const QUERY : & str = "query IntegrationsSettings($id: String!) {\n    integrationsSettings(id: $id) {\n        ...IntegrationsSettings\n    }\n}\n\nfragment IntegrationsSettings on IntegrationsSettings {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    slackIssueCreated\n    slackIssueNewComment\n    slackIssueStatusChangedDone\n    slackIssueStatusChangedAll\n    slackProjectUpdateCreated\n    slackProjectUpdateCreatedToTeam\n    slackProjectUpdateCreatedToMilestone\n    slackProjectUpdateCreatedToWorkspace\n}" ;
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
    pub struct IntegrationsSettings {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "slackIssueCreated")]
        pub slack_issue_created: Option<Boolean>,
        #[serde(rename = "slackIssueNewComment")]
        pub slack_issue_new_comment: Option<Boolean>,
        #[serde(rename = "slackIssueStatusChangedDone")]
        pub slack_issue_status_changed_done: Option<Boolean>,
        #[serde(rename = "slackIssueStatusChangedAll")]
        pub slack_issue_status_changed_all: Option<Boolean>,
        #[serde(rename = "slackProjectUpdateCreated")]
        pub slack_project_update_created: Option<Boolean>,
        #[serde(rename = "slackProjectUpdateCreatedToTeam")]
        pub slack_project_update_created_to_team: Option<Boolean>,
        #[serde(rename = "slackProjectUpdateCreatedToMilestone")]
        pub slack_project_update_created_to_milestone: Option<Boolean>,
        #[serde(rename = "slackProjectUpdateCreatedToWorkspace")]
        pub slack_project_update_created_to_workspace: Option<Boolean>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "integrationsSettings")]
        pub integrations_settings: IntegrationsSettingsIntegrationsSettings,
    }
    pub type IntegrationsSettingsIntegrationsSettings = IntegrationsSettings;
}
impl graphql_client::GraphQLQuery for IntegrationsSettings {
    type Variables = integrations_settings::Variables;
    type ResponseData = integrations_settings::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: integrations_settings::QUERY,
            operation_name: integrations_settings::OPERATION_NAME,
        }
    }
}
