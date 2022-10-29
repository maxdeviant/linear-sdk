mod instrospection_schema;

use std::fs::File;
use std::io::{BufReader, Write};
use std::process::Command;

use heck::{ToPascalCase, ToSnakeCase};
use instrospection_schema::{GraphQlTypeKind, GraphQlTypeRef, IntrospectionQuery};

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

fn render_type_name(ty: &GraphQlTypeRef) -> String {
    match ty {
        GraphQlTypeRef::Scalar { name }
        | GraphQlTypeRef::Object { name }
        | GraphQlTypeRef::Interface { name }
        | GraphQlTypeRef::Union { name }
        | GraphQlTypeRef::Enum { name }
        | GraphQlTypeRef::InputObject { name } => name.to_owned(),
        GraphQlTypeRef::NonNull(boxed) => format!("{}!", render_type_name(&boxed.of_type)),
        GraphQlTypeRef::List(boxed) => format!("[{}]", render_type_name(&boxed.of_type)),
    }
}

fn sanitize_name(name: String) -> String {
    name.replace("OAuth", "Oauth")
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

    let mut emitted_graphql_modules: Vec<String> = Vec::new();

    for field in query_fields {
        let field_type_name = resolve_type_name(&field.ty);

        let has_args = !field.args.is_empty();
        let args_list = field
            .args
            .iter()
            .map(|arg| {
                format!(
                    "${}: {}",
                    arg.name.to_snake_case(),
                    render_type_name(&arg.ty)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        let applied_args_list = field
            .args
            .iter()
            .map(|arg| format!("{}: ${}", arg.name, arg.name.to_snake_case()))
            .collect::<Vec<_>>()
            .join(", ");

        let field_type = schema
            .types
            .iter()
            .find(|ty| ty.name.as_ref() == Some(&field_type_name))
            .expect(&format!("No type found for field '{}'", field_type_name));

        let mut fragment_field_names = Vec::new();
        if let Some(sub_fields) = &field_type.fields {
            for sub_field in sub_fields {
                let sub_field_type_name = resolve_type_name(&sub_field.ty);

                let sub_field_type = schema
                    .types
                    .iter()
                    .find(|ty| ty.name.as_ref() == Some(&sub_field_type_name))
                    .expect(&format!(
                        "No type found for sub field '{}'",
                        sub_field_type_name
                    ));

                if sub_field_type.kind == GraphQlTypeKind::Scalar {
                    fragment_field_names.push(sub_field.name.clone());
                }
            }
        }

        let contents = format!(
            r#"
query {query_name}{args_list} {{
    {field_name}{applied_args_list} {{
        ...{fragment_name}
    }}
}}

fragment {fragment_name} on {fragment_name} {{
    __typename
    {fragment_fields}
}}
        "#,
            query_name = sanitize_name(field.name.clone()).to_pascal_case(),
            args_list = if has_args {
                format!("({})", args_list)
            } else {
                String::new()
            },
            applied_args_list = if has_args {
                format!("({})", applied_args_list)
            } else {
                String::new()
            },
            field_name = field.name,
            fragment_name = field_type_name.to_pascal_case(),
            fragment_fields = fragment_field_names.join("\n    ")
        );

        let rust_module_name = sanitize_name(field.name.clone()).to_snake_case();

        let mut graphql_file = File::create(format!(
            "crates/linear_sdk/src/graphql/generated/{}.graphql",
            rust_module_name
        ))?;

        graphql_file.write_all(contents.trim().as_bytes())?;

        emitted_graphql_modules.push(rust_module_name);
    }

    emitted_graphql_modules.sort_unstable();

    for emitted_graphql_module in &emitted_graphql_modules {
        let mut generate_command = Command::new("graphql-client");

        generate_command
            .arg("generate")
            .arg("--schema-path=schema.json")
            .arg("--custom-scalars-module=crate::graphql::custom_scalars")
            .arg("--response-derives=Debug")
            .arg(format!(
                "crates/linear_sdk/src/graphql/generated/{}.graphql",
                emitted_graphql_module
            ));

        generate_command.status()?;
    }

    let mut generated_module_file = File::create("crates/linear_sdk/src/graphql/generated.rs")?;

    generated_module_file.write_all(
        emitted_graphql_modules
            .into_iter()
            .map(|module_name| format!("pub mod {};", module_name))
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
    )?;

    Ok(())
}
