mod instrospection_schema;

use std::fs::File;
use std::io::{BufReader, Write};

use heck::{ToPascalCase, ToSnakeCase};
use instrospection_schema::{GraphQlTypeRef, IntrospectionQuery};

fn resolve_type_name(ty: &GraphQlTypeRef) -> &String {
    match ty {
        GraphQlTypeRef::Scalar { name }
        | GraphQlTypeRef::Object { name }
        | GraphQlTypeRef::Interface { name }
        | GraphQlTypeRef::Union { name }
        | GraphQlTypeRef::Enum { name }
        | GraphQlTypeRef::InputObject { name } => name,
        GraphQlTypeRef::NonNull(boxed) | GraphQlTypeRef::List(boxed) => {
            resolve_type_name(&boxed.of_type)
        }
    }
}

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
        let field_type_name = resolve_type_name(&field.ty);

        let field_type = schema
            .types
            .iter()
            .find(|ty| ty.name.as_ref() == Some(&field_type_name))
            .expect(&format!("No type found for field '{}'", field_type_name));

        let contents = format!(
            r#"
query {query_name}() {{
    {field_name}() {{
        ...{fragment_name}
    }}
}}

fragment {fragment_name} on {fragment_name} {{

}}
        "#,
            query_name = field.name.to_pascal_case(),
            field_name = field.name,
            fragment_name = field_type_name.to_pascal_case()
        );

        let mut graphql_file = File::create(format!(
            "crates/linear_sdk/src/graphql/{}.graphql",
            field.name.to_snake_case()
        ))?;

        graphql_file.write_all(contents.as_bytes())?;
    }

    Ok(())
}
