#![allow(clippy::all, warnings)]
pub struct Issue;
pub mod issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Issue";
    pub const QUERY : & str = "query Issue($id: String!) {\n  issue(id: $id) {\n    ...Issue\n  }\n}\n\nfragment Issue on Issue {\n  __typename\n  trashed\n  url\n  identifier\n  priorityLabel\n  previousIdentifiers\n  customerTicketCount\n  branchName\n  cycle {\n    id\n  }\n  dueDate\n  estimate\n  description\n  title\n  number\n  updatedAt\n  boardOrder\n  sortOrder\n  subIssueSortOrder\n  parent {\n    id\n  }\n  priority\n  project {\n    id\n  }\n  team {\n    id\n  }\n  archivedAt\n  createdAt\n  autoArchivedAt\n  autoClosedAt\n  canceledAt\n  completedAt\n  startedAt\n  snoozedUntilAt\n  id\n  assignee {\n    id\n  }\n  creator {\n    id\n  }\n  snoozedBy {\n    id\n  }\n  state {\n    id\n  }\n}\n" ;
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
    #[derive(Deserialize, Debug)]
    pub struct IssueCycle {
        pub id: ID,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssueParent {
        pub id: ID,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssueProject {
        pub id: ID,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssueTeam {
        pub id: ID,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssueAssignee {
        pub id: ID,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssueCreator {
        pub id: ID,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssueSnoozedBy {
        pub id: ID,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssueState {
        pub id: ID,
    }
    #[derive(Deserialize, Debug)]
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
