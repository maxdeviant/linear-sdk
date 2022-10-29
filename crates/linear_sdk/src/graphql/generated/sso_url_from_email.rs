#![allow(clippy::all, warnings)]
pub struct SsoUrlFromEmail;
pub mod sso_url_from_email {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SsoUrlFromEmail";
    pub const QUERY : & str = "query SsoUrlFromEmail($is_desktop: Boolean, $email: String!) {\n    ssoUrlFromEmail(isDesktop: $is_desktop, email: $email) {\n        ...SsoUrlFromEmailResponse\n    }\n}\n\nfragment SsoUrlFromEmailResponse on SsoUrlFromEmailResponse {\n    __typename\n    success\n    samlSsoUrl\n}" ;
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
        pub is_desktop: Option<Boolean>,
        pub email: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct SsoUrlFromEmailResponse {
        pub success: Boolean,
        #[serde(rename = "samlSsoUrl")]
        pub saml_sso_url: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "ssoUrlFromEmail")]
        pub sso_url_from_email: SsoUrlFromEmailSsoUrlFromEmail,
    }
    pub type SsoUrlFromEmailSsoUrlFromEmail = SsoUrlFromEmailResponse;
}
impl graphql_client::GraphQLQuery for SsoUrlFromEmail {
    type Variables = sso_url_from_email::Variables;
    type ResponseData = sso_url_from_email::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: sso_url_from_email::QUERY,
            operation_name: sso_url_from_email::OPERATION_NAME,
        }
    }
}
