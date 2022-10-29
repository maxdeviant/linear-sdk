#![allow(clippy::all, warnings)]
pub struct IssueImportFinishGithubOauth;
pub mod issue_import_finish_github_oauth {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssueImportFinishGithubOauth";
    pub const QUERY : & str = "query IssueImportFinishGithubOauth($code: String!) {\n    issueImportFinishGithubOAuth(code: $code) {\n        ...GithubOAuthTokenPayload\n    }\n}\n\nfragment GithubOAuthTokenPayload on GithubOAuthTokenPayload {\n    __typename\n    token\n}" ;
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
        pub code: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct GithubOAuthTokenPayload {
        pub token: Option<String>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "issueImportFinishGithubOAuth")]
        pub issue_import_finish_github_o_auth:
            IssueImportFinishGithubOauthIssueImportFinishGithubOAuth,
    }
    pub type IssueImportFinishGithubOauthIssueImportFinishGithubOAuth = GithubOAuthTokenPayload;
}
impl graphql_client::GraphQLQuery for IssueImportFinishGithubOauth {
    type Variables = issue_import_finish_github_oauth::Variables;
    type ResponseData = issue_import_finish_github_oauth::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issue_import_finish_github_oauth::QUERY,
            operation_name: issue_import_finish_github_oauth::OPERATION_NAME,
        }
    }
}
