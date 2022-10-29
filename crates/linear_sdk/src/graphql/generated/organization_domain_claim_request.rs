#![allow(clippy::all, warnings)]
pub struct OrganizationDomainClaimRequest;
pub mod organization_domain_claim_request {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "OrganizationDomainClaimRequest";
    pub const QUERY : & str = "query OrganizationDomainClaimRequest($id: String!) {\n    organizationDomainClaimRequest(id: $id) {\n        ...OrganizationDomainClaimPayload\n    }\n}\n\nfragment OrganizationDomainClaimPayload on OrganizationDomainClaimPayload {\n    __typename\n    verificationString\n}" ;
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
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct OrganizationDomainClaimPayload {
        #[serde(rename = "verificationString")]
        pub verification_string: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "organizationDomainClaimRequest")]
        pub organization_domain_claim_request:
            OrganizationDomainClaimRequestOrganizationDomainClaimRequest,
    }
    pub type OrganizationDomainClaimRequestOrganizationDomainClaimRequest =
        OrganizationDomainClaimPayload;
}
impl graphql_client::GraphQLQuery for OrganizationDomainClaimRequest {
    type Variables = organization_domain_claim_request::Variables;
    type ResponseData = organization_domain_claim_request::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: organization_domain_claim_request::QUERY,
            operation_name: organization_domain_claim_request::OPERATION_NAME,
        }
    }
}
