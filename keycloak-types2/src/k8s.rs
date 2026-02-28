use schemars::{JsonSchema, Schema};
use serde_json::Value;

/// Yields all representation types that are k8s-managed.
/// Usage: `map_k8s_patches! { T => expr using T }`
#[macro_export]
macro_rules! map_k8s_patches {
    { $type_name:ident => $expression:expr } => {
        [
            { type $type_name = $crate::RealmRepresentation; $expression },
            { type $type_name = $crate::GroupRepresentation; $expression },
            { type $type_name = $crate::ClientRepresentation; $expression },
            { type $type_name = $crate::UserRepresentation; $expression },
            { type $type_name = $crate::ClientScopeRepresentation; $expression },
            { type $type_name = $crate::ProtocolMapperRepresentation; $expression },
            { type $type_name = $crate::AuthenticationFlowRepresentation; $expression },
            { type $type_name = $crate::AuthenticatorConfigRepresentation; $expression },
            { type $type_name = $crate::RequiredActionProviderRepresentation; $expression },
            { type $type_name = $crate::OrganizationRepresentation; $expression },
            { type $type_name = $crate::ResourceRepresentation; $expression },
            { type $type_name = $crate::ScopeRepresentation; $expression },
            { type $type_name = $crate::PolicyRepresentation; $expression },
            { type $type_name = $crate::RoleRepresentation; $expression },
            { type $type_name = $crate::IdentityProviderRepresentation; $expression },
            { type $type_name = $crate::IdentityProviderMapperRepresentation; $expression },
            { type $type_name = $crate::ComponentExportRepresentation; $expression },
            { type $type_name = $crate::ApplicationRepresentation; $expression },
            { type $type_name = $crate::OAuthClientRepresentation; $expression },
        ]
    };
}

fn k8s_managed_refs() -> Vec<String> {
    map_k8s_patches! { T => format!("#/$defs/{}", <T as JsonSchema>::schema_name()) }
        .into_iter()
        .collect()
}

fn value_references_any(value: &Value, refs: &[String]) -> bool {
    if let Some(r) = value.get("$ref").and_then(Value::as_str) {
        if refs.iter().any(|managed| r == managed.as_str()) {
            return true;
        }
    }
    if let Some(items) = value.get("items") {
        if value_references_any(items, refs) {
            return true;
        }
    }
    if let Some(any_of) = value.get("anyOf").and_then(Value::as_array) {
        if any_of.iter().any(|item| value_references_any(item, refs)) {
            return true;
        }
    }
    false
}

/// Transform: removes all properties that reference k8s-managed types.
pub fn remove_k8s_managed_fields(schema: &mut Schema) {
    let refs = k8s_managed_refs();
    if let Some(properties) = schema
        .get_mut("properties")
        .and_then(Value::as_object_mut)
    {
        let to_remove: Vec<String> = properties
            .iter()
            .filter(|(_, v)| value_references_any(v, &refs))
            .map(|(k, _)| k.clone())
            .collect();
        for key in to_remove {
            properties.remove(&key);
        }
    }
}
