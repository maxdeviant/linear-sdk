#![allow(clippy::all, warnings)]
pub struct Project;
pub mod project {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Project";
    pub const QUERY : & str = "query Project($id: String!) {\n    project(id: $id) {\n        ...Project\n    }\n}\n\nfragment Project on Project {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    description\n    slugId\n    icon\n    color\n    state\n    projectUpdateRemindersPausedUntilAt\n    startDate\n    targetDate\n    startedAt\n    completedAt\n    canceledAt\n    autoArchivedAt\n    sortOrder\n    issueCountHistory\n    completedIssueCountHistory\n    scopeHistory\n    completedScopeHistory\n    inProgressScopeHistory\n    slackNewIssue\n    slackIssueComments\n    slackIssueStatuses\n    url\n    progress\n}" ;
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
    type TimelessDate = crate::graphql::custom_scalars::TimelessDate;
    type DateTime = crate::graphql::custom_scalars::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Project {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        pub description: String,
        #[serde(rename = "slugId")]
        pub slug_id: String,
        pub icon: Option<String>,
        pub color: String,
        pub state: String,
        #[serde(rename = "projectUpdateRemindersPausedUntilAt")]
        pub project_update_reminders_paused_until_at: Option<DateTime>,
        #[serde(rename = "startDate")]
        pub start_date: Option<TimelessDate>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<TimelessDate>,
        #[serde(rename = "startedAt")]
        pub started_at: Option<DateTime>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateTime>,
        #[serde(rename = "canceledAt")]
        pub canceled_at: Option<DateTime>,
        #[serde(rename = "autoArchivedAt")]
        pub auto_archived_at: Option<DateTime>,
        #[serde(rename = "sortOrder")]
        pub sort_order: Float,
        #[serde(rename = "issueCountHistory")]
        pub issue_count_history: Vec<Float>,
        #[serde(rename = "completedIssueCountHistory")]
        pub completed_issue_count_history: Vec<Float>,
        #[serde(rename = "scopeHistory")]
        pub scope_history: Vec<Float>,
        #[serde(rename = "completedScopeHistory")]
        pub completed_scope_history: Vec<Float>,
        #[serde(rename = "inProgressScopeHistory")]
        pub in_progress_scope_history: Vec<Float>,
        #[serde(rename = "slackNewIssue")]
        pub slack_new_issue: Boolean,
        #[serde(rename = "slackIssueComments")]
        pub slack_issue_comments: Boolean,
        #[serde(rename = "slackIssueStatuses")]
        pub slack_issue_statuses: Boolean,
        pub url: String,
        pub progress: Float,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub project: ProjectProject,
    }
    pub type ProjectProject = Project;
}
impl graphql_client::GraphQLQuery for Project {
    type Variables = project::Variables;
    type ResponseData = project::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project::QUERY,
            operation_name: project::OPERATION_NAME,
        }
    }
}
