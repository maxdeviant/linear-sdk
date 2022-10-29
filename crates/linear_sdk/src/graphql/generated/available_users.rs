#![allow(clippy::all, warnings)]
pub struct AvailableUsers;
pub mod available_users {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "AvailableUsers";
    pub const QUERY : & str = "query AvailableUsers {\n    availableUsers {\n        ...AuthResolverResponse\n    }\n}\n\nfragment AuthResolverResponse on AuthResolverResponse {\n    __typename\n    id\n    token\n    email\n    allowDomainAccess\n    lastUsedOrganizationId\n}" ;
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
    pub struct AuthResolverResponse {
        pub id: String,
        pub token: Option<String>,
        pub email: Option<String>,
        #[serde(rename = "allowDomainAccess")]
        pub allow_domain_access: Option<Boolean>,
        #[serde(rename = "lastUsedOrganizationId")]
        pub last_used_organization_id: Option<String>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "availableUsers")]
        pub available_users: AvailableUsersAvailableUsers,
    }
    pub type AvailableUsersAvailableUsers = AuthResolverResponse;
}
impl graphql_client::GraphQLQuery for AvailableUsers {
    type Variables = available_users::Variables;
    type ResponseData = available_users::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: available_users::QUERY,
            operation_name: available_users::OPERATION_NAME,
        }
    }
}
