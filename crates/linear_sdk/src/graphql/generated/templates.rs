#![allow(clippy::all, warnings)]
pub struct Templates;
pub mod templates {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Templates";
    pub const QUERY : & str = "query Templates {\n    templates {\n        ...Template\n    }\n}\n\nfragment Template on Template {\n    __typename\n    id\n    createdAt\n    updatedAt\n    archivedAt\n    type\n    name\n    description\n    templateData\n}" ;
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
    type JSON = crate::graphql::custom_scalars::JSON;
    type DateTime = crate::graphql::custom_scalars::DateTime;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct Template {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "updatedAt")]
        pub updated_at: DateTime,
        #[serde(rename = "archivedAt")]
        pub archived_at: Option<DateTime>,
        #[serde(rename = "type")]
        pub type_: String,
        pub name: String,
        pub description: Option<String>,
        #[serde(rename = "templateData")]
        pub template_data: JSON,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub templates: Vec<TemplatesTemplates>,
    }
    pub type TemplatesTemplates = Template;
}
impl graphql_client::GraphQLQuery for Templates {
    type Variables = templates::Variables;
    type ResponseData = templates::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: templates::QUERY,
            operation_name: templates::OPERATION_NAME,
        }
    }
}
