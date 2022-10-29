#![allow(clippy::all, warnings)]
pub struct Team;
pub mod team {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Team";
    pub const QUERY : & str = "query Team($id: String!) {\n    team(id: $id) {\n        ...Team\n    }\n}\n\nfragment Team on Team {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    key\n    description\n    icon\n    color\n    cyclesEnabled\n    cycleStartDay\n    cycleDuration\n    cycleCooldownTime\n    cycleIssueAutoAssignStarted\n    cycleIssueAutoAssignCompleted\n    cycleLockToActive\n    upcomingCycleCount\n    timezone\n    inviteHash\n    issueEstimationType\n    issueOrderingNoPriorityFirst\n    issueEstimationAllowZero\n    issueSortOrderDefaultToBottom\n    issueEstimationExtended\n    defaultIssueEstimate\n    triageEnabled\n    defaultTemplateForMembersId\n    defaultTemplateForNonMembersId\n    private\n    groupIssueHistory\n    slackNewIssue\n    slackIssueComments\n    slackIssueStatuses\n    autoClosePeriod\n    autoCloseStateId\n    autoArchivePeriod\n    cycleCalenderUrl\n}" ;
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
    pub struct Team {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        pub key: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub color: Option<String>,
        #[serde(rename = "cyclesEnabled")]
        pub cycles_enabled: Boolean,
        #[serde(rename = "cycleStartDay")]
        pub cycle_start_day: Float,
        #[serde(rename = "cycleDuration")]
        pub cycle_duration: Float,
        #[serde(rename = "cycleCooldownTime")]
        pub cycle_cooldown_time: Float,
        #[serde(rename = "cycleIssueAutoAssignStarted")]
        pub cycle_issue_auto_assign_started: Boolean,
        #[serde(rename = "cycleIssueAutoAssignCompleted")]
        pub cycle_issue_auto_assign_completed: Boolean,
        #[serde(rename = "cycleLockToActive")]
        pub cycle_lock_to_active: Boolean,
        #[serde(rename = "upcomingCycleCount")]
        pub upcoming_cycle_count: Float,
        pub timezone: String,
        #[serde(rename = "inviteHash")]
        pub invite_hash: String,
        #[serde(rename = "issueEstimationType")]
        pub issue_estimation_type: String,
        #[serde(rename = "issueOrderingNoPriorityFirst")]
        pub issue_ordering_no_priority_first: Boolean,
        #[serde(rename = "issueEstimationAllowZero")]
        pub issue_estimation_allow_zero: Boolean,
        #[serde(rename = "issueSortOrderDefaultToBottom")]
        pub issue_sort_order_default_to_bottom: Boolean,
        #[serde(rename = "issueEstimationExtended")]
        pub issue_estimation_extended: Boolean,
        #[serde(rename = "defaultIssueEstimate")]
        pub default_issue_estimate: Float,
        #[serde(rename = "triageEnabled")]
        pub triage_enabled: Boolean,
        #[serde(rename = "defaultTemplateForMembersId")]
        #[deprecated(note = "Use defaultTemplateForMembers instead")]
        pub default_template_for_members_id: Option<String>,
        #[serde(rename = "defaultTemplateForNonMembersId")]
        #[deprecated(note = "Use defaultTemplateForNonMembers instead")]
        pub default_template_for_non_members_id: Option<String>,
        pub private: Boolean,
        #[serde(rename = "groupIssueHistory")]
        pub group_issue_history: Boolean,
        #[serde(rename = "slackNewIssue")]
        pub slack_new_issue: Boolean,
        #[serde(rename = "slackIssueComments")]
        pub slack_issue_comments: Boolean,
        #[serde(rename = "slackIssueStatuses")]
        pub slack_issue_statuses: Boolean,
        #[serde(rename = "autoClosePeriod")]
        pub auto_close_period: Option<Float>,
        #[serde(rename = "autoCloseStateId")]
        pub auto_close_state_id: Option<String>,
        #[serde(rename = "autoArchivePeriod")]
        pub auto_archive_period: Float,
        #[serde(rename = "cycleCalenderUrl")]
        pub cycle_calender_url: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub team: TeamTeam,
    }
    pub type TeamTeam = Team;
}
impl graphql_client::GraphQLQuery for Team {
    type Variables = team::Variables;
    type ResponseData = team::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team::QUERY,
            operation_name: team::OPERATION_NAME,
        }
    }
}
