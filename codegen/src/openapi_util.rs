use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use syn::Ident;

pub fn get_identifier_for_path(path: &str) -> String {
    let ident = path
        .strip_prefix("/admin/realms")
        .unwrap()
        .split('/')
        .filter(|s| s.chars().next() != Some('{'))
        .map(|s| case_style::CaseStyle::guess(s).unwrap().to_snakecase())
        .map(|s| s.strip_suffix('s').map(|s| s.to_string()).unwrap_or(s))
        .collect::<Vec<_>>()
        .join("_");
    let ident = ident.trim_start_matches('_').replace('-', "_");
    if ident.is_empty() {
        "Realm".to_string()
    } else {
        let ident = CaseStyle::guess(ident).unwrap();
        ident.to_pascalcase()
    }
}

pub fn collect_path_schemas(path_item: &Value) -> Vec<Value> {
    let mut parameters = path_item["parameters"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    let response_parameters = path_item
        .as_object()
        .unwrap()
        .values()
        .filter_map(|x| x.get("parameters"))
        .flat_map(|x| x.as_array().cloned().unwrap_or_default().into_iter())
        .filter(|x| x["in"] == "path")
        .collect::<Vec<_>>();

    parameters.extend(response_parameters);

    parameters
}

pub fn impl_response(
    trait_name: &str,
    ident: &Ident,
    operation: &Value,
) -> TokenStream {
    let responses = &operation["responses"];
    let trait_ident = format_ident!("{}", trait_name);

    let rtype = if let Some(response200) = responses.get("200") {
        let Some(content) = response200.get("content") else {
            return quote! {};
        };
        let schema =
            content.as_object().and_then(|x| x.values().next()).unwrap();

        get_type(&schema["schema"])
    } else {
        quote! { () }
    };

    quote! {
        impl #trait_ident for #ident {
            type Response = #rtype;
        }
    }
}

pub fn get_type(schema: &Value) -> TokenStream {
    if let Some(r) = schema["$ref"].as_str() {
        let (_, r) = r.rsplit_once('/').unwrap();
        let r = format_ident!("{}", r);
        return quote! { crate::schema_gen::#r };
    }
    match schema["type"].as_str() {
        Some("boolean") => {
            quote! { bool }
        }
        Some("string") => {
            quote! { String }
        }
        Some("integer") => {
            quote! { i64 }
        }
        Some("array") => {
            let items = &schema["items"];
            let item_type = get_type(items);
            quote! { Vec<#item_type> }
        }
        Some("object") => {
            quote! { HashMap<String, Value> }
        }
        _ => panic!("Unsupported schema type"),
    }
}
