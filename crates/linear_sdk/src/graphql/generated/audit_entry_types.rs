#![allow(clippy::all, warnings)]
pub struct AuditEntryTypes;
pub mod audit_entry_types {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "AuditEntryTypes";
    pub const QUERY : & str = "query AuditEntryTypes {\n    auditEntryTypes {\n        ...AuditEntryType\n    }\n}\n\nfragment AuditEntryType on AuditEntryType {\n    __typename\n    type\n    description\n}" ;
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
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct AuditEntryType {
        #[serde(rename = "type")]
        pub type_: String,
        pub description: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "auditEntryTypes")]
        pub audit_entry_types: Vec<AuditEntryTypesAuditEntryTypes>,
    }
    pub type AuditEntryTypesAuditEntryTypes = AuditEntryType;
}
impl graphql_client::GraphQLQuery for AuditEntryTypes {
    type Variables = audit_entry_types::Variables;
    type ResponseData = audit_entry_types::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: audit_entry_types::QUERY,
            operation_name: audit_entry_types::OPERATION_NAME,
        }
    }
}
