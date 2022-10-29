#![allow(clippy::all, warnings)]
pub struct AuthorizedApplications;
pub mod authorized_applications {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "AuthorizedApplications";
    pub const QUERY : & str = "query AuthorizedApplications {\n    authorizedApplications {\n        ...AuthorizedApplication\n    }\n}\n\nfragment AuthorizedApplication on AuthorizedApplication {\n    __typename\n    name\n    imageUrl\n    scope\n    appId\n    clientId\n    webhooksEnabled\n}" ;
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
    pub struct AuthorizedApplication {
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
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "authorizedApplications")]
        pub authorized_applications: Vec<AuthorizedApplicationsAuthorizedApplications>,
    }
    pub type AuthorizedApplicationsAuthorizedApplications = AuthorizedApplication;
}
impl graphql_client::GraphQLQuery for AuthorizedApplications {
    type Variables = authorized_applications::Variables;
    type ResponseData = authorized_applications::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: authorized_applications::QUERY,
            operation_name: authorized_applications::OPERATION_NAME,
        }
    }
}
