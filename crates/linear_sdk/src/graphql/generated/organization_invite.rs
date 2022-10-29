#![allow(clippy::all, warnings)]
pub struct OrganizationInvite;
pub mod organization_invite {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "OrganizationInvite";
    pub const QUERY : & str = "query OrganizationInvite($id: String!) {\n    organizationInvite(id: $id) {\n        ...OrganizationInvite\n    }\n}\n\nfragment OrganizationInvite on OrganizationInvite {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    email\n    external\n    acceptedAt\n    expiresAt\n}" ;
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
    pub struct OrganizationInvite {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub email: String,
        pub external: Boolean,
        #[serde(rename = "acceptedAt")]
        pub accepted_at: Option<DateTime>,
        #[serde(rename = "expiresAt")]
        pub expires_at: Option<DateTime>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "organizationInvite")]
        pub organization_invite: OrganizationInviteOrganizationInvite,
    }
    pub type OrganizationInviteOrganizationInvite = OrganizationInvite;
}
impl graphql_client::GraphQLQuery for OrganizationInvite {
    type Variables = organization_invite::Variables;
    type ResponseData = organization_invite::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: organization_invite::QUERY,
            operation_name: organization_invite::OPERATION_NAME,
        }
    }
}
