#![allow(clippy::all, warnings)]
pub struct TeamMembership;
pub mod team_membership {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TeamMembership";
    pub const QUERY : & str = "query TeamMembership($id: String!) {\n    teamMembership(id: $id) {\n        ...TeamMembership\n    }\n}\n\nfragment TeamMembership on TeamMembership {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    owner\n    sortOrder\n}" ;
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
    pub struct TeamMembership {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub owner: Option<Boolean>,
        #[serde(rename = "sortOrder")]
        pub sort_order: Float,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "teamMembership")]
        pub team_membership: TeamMembershipTeamMembership,
    }
    pub type TeamMembershipTeamMembership = TeamMembership;
}
impl graphql_client::GraphQLQuery for TeamMembership {
    type Variables = team_membership::Variables;
    type ResponseData = team_membership::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team_membership::QUERY,
            operation_name: team_membership::OPERATION_NAME,
        }
    }
}
