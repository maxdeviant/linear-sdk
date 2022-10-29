#![allow(clippy::all, warnings)]
pub struct ApplicationWithAuthorization;
pub mod application_with_authorization {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ApplicationWithAuthorization";
    pub const QUERY : & str = "query ApplicationWithAuthorization($actor: String, $redirect_uri: String, $scope: [String!]!, $client_id: String!) {\n    applicationWithAuthorization(actor: $actor, redirectUri: $redirect_uri, scope: $scope, clientId: $client_id) {\n        ...UserAuthorizedApplication\n    }\n}\n\nfragment UserAuthorizedApplication on UserAuthorizedApplication {\n    __typename\n    id\n    clientId\n    name\n    description\n    developer\n    developerUrl\n    imageUrl\n    isAuthorized\n    createdByLinear\n    webhooksEnabled\n    approvalErrorCode\n}" ;
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
    pub struct Variables {
        pub actor: Option<String>,
        pub redirect_uri: Option<String>,
        pub scope: Vec<String>,
        pub client_id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct UserAuthorizedApplication {
        pub id: String,
        #[serde(rename = "clientId")]
        pub client_id: String,
        pub name: String,
        pub description: Option<String>,
        pub developer: String,
        #[serde(rename = "developerUrl")]
        pub developer_url: String,
        #[serde(rename = "imageUrl")]
        pub image_url: Option<String>,
        #[serde(rename = "isAuthorized")]
        pub is_authorized: Boolean,
        #[serde(rename = "createdByLinear")]
        pub created_by_linear: Boolean,
        #[serde(rename = "webhooksEnabled")]
        pub webhooks_enabled: Boolean,
        #[serde(rename = "approvalErrorCode")]
        pub approval_error_code: Option<String>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "applicationWithAuthorization")]
        pub application_with_authorization:
            ApplicationWithAuthorizationApplicationWithAuthorization,
    }
    pub type ApplicationWithAuthorizationApplicationWithAuthorization = UserAuthorizedApplication;
}
impl graphql_client::GraphQLQuery for ApplicationWithAuthorization {
    type Variables = application_with_authorization::Variables;
    type ResponseData = application_with_authorization::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: application_with_authorization::QUERY,
            operation_name: application_with_authorization::OPERATION_NAME,
        }
    }
}
