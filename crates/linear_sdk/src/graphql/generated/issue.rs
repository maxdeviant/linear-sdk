#![allow(clippy::all, warnings)]
pub struct Issue;
pub mod issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Issue";
    pub const QUERY : & str = "query Issue($id: String!) {\n    issue(id: $id) {\n        ...Issue\n    }\n}\n\nfragment Issue on Issue {\n    archivedAt\n    autoArchivedAt\n    autoClosedAt\n    boardOrder\n    branchName\n    canceledAt\n    completedAt\n    createdAt\n    customerTicketCount\n    description\n    dueDate\n    estimate\n    id\n    identifier\n    number\n    previousIdentifiers\n    priority\n    priorityLabel\n    snoozedUntilAt\n    sortOrder\n    startedAt\n    subIssueSortOrder\n    title\n    trashed\n    triagedAt\n    updatedAt\n    url\n}" ;
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
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "autoArchivedAt")]
        pub auto_archived_at: Option<DateTime>,
        #[serde(rename = "autoClosedAt")]
        pub auto_closed_at: Option<DateTime>,
        #[serde(rename = "boardOrder")]
        #[deprecated(note = "Will be removed in near future, please use `sortOrder` instead")]
        pub board_order: Float,
        #[serde(rename = "branchName")]
        pub branch_name: String,
        #[serde(rename = "canceledAt")]
        pub canceled_at: Option<DateTime>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateTime>,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "customerTicketCount")]
        pub customer_ticket_count: Int,
        pub description: Option<String>,
        #[serde(rename = "dueDate")]
        pub due_date: Option<TimelessDate>,
        pub estimate: Option<Float>,
        pub id: ID,
        pub identifier: String,
        pub number: Float,
        #[serde(rename = "previousIdentifiers")]
        pub previous_identifiers: Vec<String>,
        pub priority: Float,
        #[serde(rename = "priorityLabel")]
        pub priority_label: String,
        #[serde(rename = "snoozedUntilAt")]
        pub snoozed_until_at: Option<DateTime>,
        #[serde(rename = "sortOrder")]
        pub sort_order: Float,
        #[serde(rename = "startedAt")]
        pub started_at: Option<DateTime>,
        #[serde(rename = "subIssueSortOrder")]
        pub sub_issue_sort_order: Option<Float>,
        pub title: String,
        pub trashed: Option<Boolean>,
        #[serde(rename = "triagedAt")]
        pub triaged_at: Option<DateTime>,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        pub url: String,
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
