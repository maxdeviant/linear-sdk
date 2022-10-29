#![allow(clippy::all, warnings)]
pub struct OrganizationExists;
pub mod organization_exists {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "OrganizationExists";
    pub const QUERY : & str = "query OrganizationExists($url_key: String!) {\n    organizationExists(urlKey: $url_key) {\n        ...OrganizationExistsPayload\n    }\n}\n\nfragment OrganizationExistsPayload on OrganizationExistsPayload {\n    __typename\n    success\n    exists\n}" ;
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
        pub url_key: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct OrganizationExistsPayload {
        pub success: Boolean,
        pub exists: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "organizationExists")]
        pub organization_exists: OrganizationExistsOrganizationExists,
    }
    pub type OrganizationExistsOrganizationExists = OrganizationExistsPayload;
}
impl graphql_client::GraphQLQuery for OrganizationExists {
    type Variables = organization_exists::Variables;
    type ResponseData = organization_exists::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: organization_exists::QUERY,
            operation_name: organization_exists::OPERATION_NAME,
        }
    }
}
