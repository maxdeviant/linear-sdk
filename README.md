# linear_sdk

A Linear SDK for Rust

## Development

### Update GraphQL Schema

```
graphql-client introspect-schema https://api.linear.app/graphql --output schema.json
```

### Run GraphQL Codegen

```
graphql-client generate --schema-path=schema.json --custom-scalars-module='crate::graphql::custom_scalars' --output-directory src/graphql/generated/ src/graphql/issue.graphql
```
