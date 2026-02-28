use schemars::schema::RootSchema;
use serde_json::Value;
use std::fs::File;

/// Load an OpenAPI spec and convert `components.schemas` into a
/// schemars 0.8 `RootSchema` suitable for typify.
pub fn load_openapi_as_root_schema(
    path: &str,
) -> Result<RootSchema, Box<dyn std::error::Error>> {
    let openapi: Value = serde_json::from_reader(File::open(path)?)?;

    let schemas = openapi
        .pointer("/components/schemas")
        .ok_or("missing /components/schemas in OpenAPI spec")?
        .clone();

    // Build a minimal JSON Schema document with `definitions` populated
    // from the OpenAPI `components.schemas`.
    let mut root = serde_json::json!({
        "type": "object",
        "definitions": schemas,
    });

    // Rewrite all `$ref` pointers from OpenAPI style to JSON Schema style:
    //   #/components/schemas/Foo  ->  #/definitions/Foo
    rewrite_refs(&mut root);

    let root_schema: RootSchema = serde_json::from_value(root)?;
    Ok(root_schema)
}

/// Recursively rewrite `$ref` values from `#/components/schemas/X`
/// to `#/definitions/X`.
fn rewrite_refs(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(r)) = map.get_mut("$ref") {
                if let Some(suffix) = r.strip_prefix("#/components/schemas/") {
                    *r = format!("#/definitions/{suffix}");
                }
            }
            for v in map.values_mut() {
                rewrite_refs(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                rewrite_refs(v);
            }
        }
        _ => {}
    }
}
