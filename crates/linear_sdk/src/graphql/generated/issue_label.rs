#![allow(clippy::all, warnings)]
pub struct IssueLabel;
pub mod issue_label {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssueLabel";
    pub const QUERY : & str = "query IssueLabel($id: String!) {\n    issueLabel(id: $id) {\n        ...IssueLabel\n    }\n}\n\nfragment IssueLabel on IssueLabel {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    description\n    color\n}" ;
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
    pub struct IssueLabel {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        pub description: Option<String>,
        pub color: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "issueLabel")]
        pub issue_label: IssueLabelIssueLabel,
    }
    pub type IssueLabelIssueLabel = IssueLabel;
}
impl graphql_client::GraphQLQuery for IssueLabel {
    type Variables = issue_label::Variables;
    type ResponseData = issue_label::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issue_label::QUERY,
            operation_name: issue_label::OPERATION_NAME,
        }
    }
}
