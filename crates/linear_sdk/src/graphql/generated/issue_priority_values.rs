#![allow(clippy::all, warnings)]
pub struct IssuePriorityValues;
pub mod issue_priority_values {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssuePriorityValues";
    pub const QUERY : & str = "query IssuePriorityValues {\n    issuePriorityValues {\n        ...IssuePriorityValue\n    }\n}\n\nfragment IssuePriorityValue on IssuePriorityValue {\n    __typename\n    priority\n    label\n}" ;
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
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct IssuePriorityValue {
        pub priority: Int,
        pub label: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "issuePriorityValues")]
        pub issue_priority_values: Vec<IssuePriorityValuesIssuePriorityValues>,
    }
    pub type IssuePriorityValuesIssuePriorityValues = IssuePriorityValue;
}
impl graphql_client::GraphQLQuery for IssuePriorityValues {
    type Variables = issue_priority_values::Variables;
    type ResponseData = issue_priority_values::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issue_priority_values::QUERY,
            operation_name: issue_priority_values::OPERATION_NAME,
        }
    }
}
