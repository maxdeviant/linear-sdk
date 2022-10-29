#![allow(clippy::all, warnings)]
pub struct WorkflowState;
pub mod workflow_state {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "WorkflowState";
    pub const QUERY : & str = "query WorkflowState($id: String!) {\n    workflowState(id: $id) {\n        ...WorkflowState\n    }\n}\n\nfragment WorkflowState on WorkflowState {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    color\n    description\n    position\n    type\n}" ;
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
    pub struct WorkflowState {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        pub color: String,
        pub description: Option<String>,
        pub position: Float,
        #[serde(rename = "type")]
        pub type_: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "workflowState")]
        pub workflow_state: WorkflowStateWorkflowState,
    }
    pub type WorkflowStateWorkflowState = WorkflowState;
}
impl graphql_client::GraphQLQuery for WorkflowState {
    type Variables = workflow_state::Variables;
    type ResponseData = workflow_state::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: workflow_state::QUERY,
            operation_name: workflow_state::OPERATION_NAME,
        }
    }
}
