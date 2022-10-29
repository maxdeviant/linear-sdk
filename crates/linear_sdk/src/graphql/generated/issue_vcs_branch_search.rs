#![allow(clippy::all, warnings)]
pub struct IssueVcsBranchSearch;
pub mod issue_vcs_branch_search {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssueVcsBranchSearch";
    pub const QUERY : & str = "query IssueVcsBranchSearch($branch_name: String!) {\n    issueVcsBranchSearch(branchName: $branch_name) {\n        ...Issue\n    }\n}\n\nfragment Issue on Issue {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    number\n    title\n    description\n    priority\n    estimate\n    boardOrder\n    sortOrder\n    startedAt\n    completedAt\n    triagedAt\n    canceledAt\n    autoClosedAt\n    autoArchivedAt\n    dueDate\n    trashed\n    snoozedUntilAt\n    previousIdentifiers\n    subIssueSortOrder\n    priorityLabel\n    identifier\n    url\n    branchName\n    customerTicketCount\n}" ;
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
        pub branch_name: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Issue {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub number: Float,
        pub title: String,
        pub description: Option<String>,
        pub priority: Float,
        pub estimate: Option<Float>,
        #[serde(rename = "boardOrder")]
        #[deprecated(note = "Will be removed in near future, please use `sortOrder` instead")]
        pub board_order: Float,
        #[serde(rename = "sortOrder")]
        pub sort_order: Float,
        #[serde(rename = "startedAt")]
        pub started_at: Option<DateTime>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateTime>,
        #[serde(rename = "triagedAt")]
        pub triaged_at: Option<DateTime>,
        #[serde(rename = "canceledAt")]
        pub canceled_at: Option<DateTime>,
        #[serde(rename = "autoClosedAt")]
        pub auto_closed_at: Option<DateTime>,
        #[serde(rename = "autoArchivedAt")]
        pub auto_archived_at: Option<DateTime>,
        #[serde(rename = "dueDate")]
        pub due_date: Option<TimelessDate>,
        pub trashed: Option<Boolean>,
        #[serde(rename = "snoozedUntilAt")]
        pub snoozed_until_at: Option<DateTime>,
        #[serde(rename = "previousIdentifiers")]
        pub previous_identifiers: Vec<String>,
        #[serde(rename = "subIssueSortOrder")]
        pub sub_issue_sort_order: Option<Float>,
        #[serde(rename = "priorityLabel")]
        pub priority_label: String,
        pub identifier: String,
        pub url: String,
        #[serde(rename = "branchName")]
        pub branch_name: String,
        #[serde(rename = "customerTicketCount")]
        pub customer_ticket_count: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "issueVcsBranchSearch")]
        pub issue_vcs_branch_search: Option<IssueVcsBranchSearchIssueVcsBranchSearch>,
    }
    pub type IssueVcsBranchSearchIssueVcsBranchSearch = Issue;
}
impl graphql_client::GraphQLQuery for IssueVcsBranchSearch {
    type Variables = issue_vcs_branch_search::Variables;
    type ResponseData = issue_vcs_branch_search::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issue_vcs_branch_search::QUERY,
            operation_name: issue_vcs_branch_search::OPERATION_NAME,
        }
    }
}
