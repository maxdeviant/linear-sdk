#![allow(clippy::all, warnings)]
pub struct CustomView;
pub mod custom_view {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CustomView";
    pub const QUERY : & str = "query CustomView($id: String!) {\n    customView(id: $id) {\n        ...CustomView\n    }\n}\n\nfragment CustomView on CustomView {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    name\n    description\n    icon\n    color\n    filters\n    filterData\n    shared\n}" ;
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
    type JSONObject = crate::graphql::custom_scalars::JSONObject;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct CustomView {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub color: Option<String>,
        #[deprecated(note = "Will be replaced by `filterData` in a future update")]
        pub filters: JSONObject,
        #[serde(rename = "filterData")]
        pub filter_data: JSONObject,
        pub shared: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "customView")]
        pub custom_view: CustomViewCustomView,
    }
    pub type CustomViewCustomView = CustomView;
}
impl graphql_client::GraphQLQuery for CustomView {
    type Variables = custom_view::Variables;
    type ResponseData = custom_view::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: custom_view::QUERY,
            operation_name: custom_view::OPERATION_NAME,
        }
    }
}
