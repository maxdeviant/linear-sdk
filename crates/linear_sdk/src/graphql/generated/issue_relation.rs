#![allow(clippy::all, warnings)]
pub struct IssueRelation;
pub mod issue_relation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssueRelation";
    pub const QUERY : & str = "query IssueRelation($id: String!) {\n    issueRelation(id: $id) {\n        ...IssueRelation\n    }\n}\n\nfragment IssueRelation on IssueRelation {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    type\n}" ;
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
    pub struct IssueRelation {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "type")]
        pub type_: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "issueRelation")]
        pub issue_relation: IssueRelationIssueRelation,
    }
    pub type IssueRelationIssueRelation = IssueRelation;
}
impl graphql_client::GraphQLQuery for IssueRelation {
    type Variables = issue_relation::Variables;
    type ResponseData = issue_relation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issue_relation::QUERY,
            operation_name: issue_relation::OPERATION_NAME,
        }
    }
}
