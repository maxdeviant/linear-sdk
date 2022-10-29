#![allow(clippy::all, warnings)]
pub struct FigmaEmbedInfo;
pub mod figma_embed_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "FigmaEmbedInfo";
    pub const QUERY : & str = "query FigmaEmbedInfo($node_id: String, $file_id: String!) {\n    figmaEmbedInfo(nodeId: $node_id, fileId: $file_id) {\n        ...FigmaEmbedPayload\n    }\n}\n\nfragment FigmaEmbedPayload on FigmaEmbedPayload {\n    __typename\n    success\n}" ;
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
        pub node_id: Option<String>,
        pub file_id: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct FigmaEmbedPayload {
        pub success: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "figmaEmbedInfo")]
        pub figma_embed_info: FigmaEmbedInfoFigmaEmbedInfo,
    }
    pub type FigmaEmbedInfoFigmaEmbedInfo = FigmaEmbedPayload;
}
impl graphql_client::GraphQLQuery for FigmaEmbedInfo {
    type Variables = figma_embed_info::Variables;
    type ResponseData = figma_embed_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: figma_embed_info::QUERY,
            operation_name: figma_embed_info::OPERATION_NAME,
        }
    }
}
