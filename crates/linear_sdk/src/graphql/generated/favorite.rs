#![allow(clippy::all, warnings)]
pub struct Favorite;
pub mod favorite {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Favorite";
    pub const QUERY : & str = "query Favorite($id: String!) {\n    favorite(id: $id) {\n        ...Favorite\n    }\n}\n\nfragment Favorite on Favorite {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    type\n    folderName\n    predefinedViewType\n    sortOrder\n}" ;
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
    pub struct Favorite {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "type")]
        pub type_: String,
        #[serde(rename = "folderName")]
        pub folder_name: Option<String>,
        #[serde(rename = "predefinedViewType")]
        pub predefined_view_type: Option<String>,
        #[serde(rename = "sortOrder")]
        pub sort_order: Float,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub favorite: FavoriteFavorite,
    }
    pub type FavoriteFavorite = Favorite;
}
impl graphql_client::GraphQLQuery for Favorite {
    type Variables = favorite::Variables;
    type ResponseData = favorite::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: favorite::QUERY,
            operation_name: favorite::OPERATION_NAME,
        }
    }
}
