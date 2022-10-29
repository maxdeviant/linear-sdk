#![allow(clippy::all, warnings)]
pub struct Issue;
pub mod issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Issue";
    pub const QUERY : & str = "query Issue($id: String!) {\n  issue(id: $id) {\n    ...Issue\n  }\n}\n\n# An issue.\nfragment Issue on Issue {\n  __typename\n  # A flag that indicates whether the issue is in the trash bin.\n  trashed\n  # Issue URL.\n  url\n  # Issue's human readable identifier (e.g. ENG-123).\n  identifier\n  # Label for the priority.\n  priorityLabel\n  # Previous identifiers of the issue if it has been moved between teams.\n  previousIdentifiers\n  # Returns the number of Attachment resources which are created by customer support ticketing systems (e.g. Zendesk).\n  customerTicketCount\n  # Suggested branch name for the issue.\n  branchName\n  # The cycle that the issue is associated with.\n  cycle {\n    id\n  }\n  # The date at which the issue is due.\n  dueDate\n  # The estimate of the complexity of the issue..\n  estimate\n  # The issue's description in markdown format.\n  description\n  # The issue's title.\n  title\n  # The issue's unique number.\n  number\n  # The last time at which the entity was updated. This is the same as the creation time if the\n  #     entity hasn't been updated after creation.\n  updatedAt\n  # The order of the item in its column on the board.\n  boardOrder\n  # The order of the item in relation to other items in the organization.\n  sortOrder\n  # The order of the item in the sub-issue list. Only set if the issue has a parent.\n  subIssueSortOrder\n  # The parent of the issue.\n  parent {\n    id\n  }\n  # The priority of the issue.\n  priority\n  # The project that the issue is associated with.\n  project {\n    id\n  }\n  # The team that the issue is associated with.\n  team {\n    id\n  }\n  # The time at which the entity was archived. Null if the entity has not been archived.\n  archivedAt\n  # The time at which the entity was created.\n  createdAt\n  # The time at which the issue was automatically archived by the auto pruning process.\n  autoArchivedAt\n  # The time at which the issue was automatically closed by the auto pruning process.\n  autoClosedAt\n  # The time at which the issue was moved into canceled state.\n  canceledAt\n  # The time at which the issue was moved into completed state.\n  completedAt\n  # The time at which the issue was moved into started state.\n  startedAt\n  # The time until an issue will be snoozed in Triage view.\n  snoozedUntilAt\n  # The unique identifier of the entity.\n  id\n  # The user to whom the issue is assigned to.\n  assignee {\n    id\n  }\n  # The user who created the issue.\n  creator {\n    id\n  }\n  # The user who snoozed the issue.\n  snoozedBy {\n    id\n  }\n  # The workflow state that the issue is associated with.\n  state {\n    id\n  }\n}\n" ;
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
    type DateTime = crate::custom_scalars::DateTime;
    type TimelessDate = crate::custom_scalars::TimelessDate;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct Issue {
        pub trashed: Option<Boolean>,
        pub url: String,
        pub identifier: String,
        #[serde(rename = "priorityLabel")]
        pub priority_label: String,
        #[serde(rename = "previousIdentifiers")]
        pub previous_identifiers: Vec<String>,
        #[serde(rename = "customerTicketCount")]
        pub customer_ticket_count: Int,
        #[serde(rename = "branchName")]
        pub branch_name: String,
        pub cycle: Option<IssueCycle>,
        #[serde(rename = "dueDate")]
        pub due_date: Option<TimelessDate>,
        pub estimate: Option<Float>,
        pub description: Option<String>,
        pub title: String,
        pub number: Float,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "boardOrder")]
        #[deprecated(note = "Will be removed in near future, please use `sortOrder` instead")]
        pub board_order: Float,
        #[serde(rename = "sortOrder")]
        pub sort_order: Float,
        #[serde(rename = "subIssueSortOrder")]
        pub sub_issue_sort_order: Option<Float>,
        pub parent: Option<IssueParent>,
        pub priority: Float,
        pub project: Option<IssueProject>,
        pub team: IssueTeam,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "autoArchivedAt")]
        pub auto_archived_at: Option<DateTime>,
        #[serde(rename = "autoClosedAt")]
        pub auto_closed_at: Option<DateTime>,
        #[serde(rename = "canceledAt")]
        pub canceled_at: Option<DateTime>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateTime>,
        #[serde(rename = "startedAt")]
        pub started_at: Option<DateTime>,
        #[serde(rename = "snoozedUntilAt")]
        pub snoozed_until_at: Option<DateTime>,
        pub id: ID,
        pub assignee: Option<IssueAssignee>,
        pub creator: Option<IssueCreator>,
        #[serde(rename = "snoozedBy")]
        pub snoozed_by: Option<IssueSnoozedBy>,
        pub state: IssueState,
    }
    #[derive(Deserialize)]
    pub struct IssueCycle {
        pub id: ID,
    }
    #[derive(Deserialize)]
    pub struct IssueParent {
        pub id: ID,
    }
    #[derive(Deserialize)]
    pub struct IssueProject {
        pub id: ID,
    }
    #[derive(Deserialize)]
    pub struct IssueTeam {
        pub id: ID,
    }
    #[derive(Deserialize)]
    pub struct IssueAssignee {
        pub id: ID,
    }
    #[derive(Deserialize)]
    pub struct IssueCreator {
        pub id: ID,
    }
    #[derive(Deserialize)]
    pub struct IssueSnoozedBy {
        pub id: ID,
    }
    #[derive(Deserialize)]
    pub struct IssueState {
        pub id: ID,
    }
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub issue: IssueIssue,
    }
    pub type IssueIssue = Issue;
}
impl graphql_client::GraphQLQuery for Issue {
    type Variables = issue::Variables;
    type ResponseData = issue::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issue::QUERY,
            operation_name: issue::OPERATION_NAME,
        }
    }
}
