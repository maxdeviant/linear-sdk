#![allow(clippy::all, warnings)]
pub struct OrganizationInviteDetails;
pub mod organization_invite_details {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "OrganizationInviteDetails";
    pub const QUERY : & str = "query OrganizationInviteDetails($id: String!) {\n    organizationInviteDetails(id: $id) {\n        ...OrganizationInviteDetailsPayload\n    }\n}\n\nfragment OrganizationInviteDetailsPayload on OrganizationInviteDetailsPayload {\n    __typename\n    inviter\n    email\n    createdAt\n    organizationName\n    organizationId\n    organizationLogoUrl\n    accepted\n    expired\n}" ;
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
    pub struct OrganizationInviteDetailsPayload {
        pub inviter: String,
        pub email: String,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "organizationName")]
        pub organization_name: String,
        #[serde(rename = "organizationId")]
        pub organization_id: String,
        #[serde(rename = "organizationLogoUrl")]
        pub organization_logo_url: Option<String>,
        pub accepted: Boolean,
        pub expired: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "organizationInviteDetails")]
        pub organization_invite_details: OrganizationInviteDetailsOrganizationInviteDetails,
    }
    pub type OrganizationInviteDetailsOrganizationInviteDetails = OrganizationInviteDetailsPayload;
}
impl graphql_client::GraphQLQuery for OrganizationInviteDetails {
    type Variables = organization_invite_details::Variables;
    type ResponseData = organization_invite_details::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: organization_invite_details::QUERY,
            operation_name: organization_invite_details::OPERATION_NAME,
        }
    }
}
