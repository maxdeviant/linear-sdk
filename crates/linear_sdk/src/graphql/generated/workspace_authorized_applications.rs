#![allow(clippy::all, warnings)]
pub struct WorkspaceAuthorizedApplications;
pub mod workspace_authorized_applications {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "WorkspaceAuthorizedApplications";
    pub const QUERY : & str = "query WorkspaceAuthorizedApplications {\n    workspaceAuthorizedApplications {\n        ...WorkspaceAuthorizedApplication\n    }\n}\n\nfragment WorkspaceAuthorizedApplication on WorkspaceAuthorizedApplication {\n    __typename\n    name\n    imageUrl\n    scope\n    appId\n    clientId\n    webhooksEnabled\n    totalMembers\n}" ;
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
    pub struct WorkspaceAuthorizedApplication {
        pub name: String,
        #[serde(rename = "imageUrl")]
        pub image_url: Option<String>,
        pub scope: Vec<String>,
        #[serde(rename = "appId")]
        pub app_id: String,
        #[serde(rename = "clientId")]
        pub client_id: String,
        #[serde(rename = "webhooksEnabled")]
        pub webhooks_enabled: Boolean,
        #[serde(rename = "totalMembers")]
        pub total_members: Float,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "workspaceAuthorizedApplications")]
        pub workspace_authorized_applications:
            Vec<WorkspaceAuthorizedApplicationsWorkspaceAuthorizedApplications>,
    }
    pub type WorkspaceAuthorizedApplicationsWorkspaceAuthorizedApplications =
        WorkspaceAuthorizedApplication;
}
impl graphql_client::GraphQLQuery for WorkspaceAuthorizedApplications {
    type Variables = workspace_authorized_applications::Variables;
    type ResponseData = workspace_authorized_applications::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: workspace_authorized_applications::QUERY,
            operation_name: workspace_authorized_applications::OPERATION_NAME,
        }
    }
}
