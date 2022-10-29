use std::fs::File;
use std::io::{BufReader, Write};

use heck::{ToPascalCase, ToSnakeCase};
use instrospection_schema::IntrospectionQuery;

mod instrospection_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_file = File::open("schema.json")?;
    let buf_reader = BufReader::new(schema_file);

    let schema_query: IntrospectionQuery = serde_json::from_reader(buf_reader)?;

    let schema = schema_query.data.schema;

    let query_name = schema.query_type.name;

    let query_type = schema
        .types
        .iter()
        .find(|ty| ty.name.as_ref() == Some(&query_name))
        .expect("No Query type found");

    let query_fields = query_type.fields.as_ref().expect("Query has no fields");

    for field in query_fields {
        let contents = format!(
            r#"
query {query_name}() {{

}}
        "#,
            query_name = field.name.to_pascal_case()
        );

        let mut graphql_file = File::create(format!(
            "crates/linear_sdk/src/graphql/{}.graphql",
            field.name.to_snake_case()
        ))?;

        graphql_file.write_all(contents.as_bytes())?;
    }

    Ok(())
}
