#![allow(clippy::all, warnings)]
pub struct ApplicationInfo;
pub mod application_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ApplicationInfo";
    pub const QUERY : & str = "query ApplicationInfo($client_id: String!) {\n    applicationInfo(clientId: $client_id) {\n        ...Application\n    }\n}\n\nfragment Application on Application {\n    __typename\n    id\n    clientId\n    name\n    description\n    developer\n    developerUrl\n    imageUrl\n}" ;
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
        pub client_id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct Application {
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
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "applicationInfo")]
        pub application_info: ApplicationInfoApplicationInfo,
    }
    pub type ApplicationInfoApplicationInfo = Application;
}
impl graphql_client::GraphQLQuery for ApplicationInfo {
    type Variables = application_info::Variables;
    type ResponseData = application_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: application_info::QUERY,
            operation_name: application_info::OPERATION_NAME,
        }
    }
}
