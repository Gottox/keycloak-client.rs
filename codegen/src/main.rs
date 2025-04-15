//mod openapi_util;
mod schema_utils;

//use case_style::CaseStyle;
use clap::Parser;
use reqwest::blocking::Client;
use schema_utils::cleanup_schema;
use schemars::schema::RootSchema;
use std::{
    fs::{self, File},
    io,
    path::Path,
};
use typify::{TypeSpace, TypeSpaceSettings};

macro_rules! root_dir {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/..")
    };
    ($path:literal) => {
        concat!(root_dir!(), "/", $path)
    };
}
const SCHEMAS_DIR: &str = root_dir!("schemas");
const GENERATE_DIR: &str = root_dir!("keycloak-types/src");

#[derive(Debug, Parser, Clone)]
struct Opts {
    #[clap(long)]
    update: bool,
    #[clap(long)]
    generate: bool,
}

fn schema_path(version: &str) -> String {
    format!("{}/jsonschema-{}.json", SCHEMAS_DIR, version)
}

fn openapi_path(version: &str) -> String {
    format!("{}/openapi-{}.json", SCHEMAS_DIR, version)
}

fn download(
    url: &str,
    out_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let out_path = Path::new(SCHEMAS_DIR).join(out_file);
    let mut response = client.get(url).send()?.error_for_status()?;
    let mut out_file = File::create(out_path)?;
    io::copy(&mut response, &mut out_file)?;
    Ok(())
}

fn update(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    // For Routes
    let openapi_url = format!(
        "https://www.keycloak.org/docs-api/{version}/rest-api/openapi.json"
    );
    // For Representations
    let jsonschema_url = format!(
        "https://jirutka.github.io/keycloak-json-schema/keycloak-all-{version}.json"
    );

    fs::create_dir_all(SCHEMAS_DIR)?;
    for entry in fs::read_dir(SCHEMAS_DIR)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some("json".as_ref()) {
            fs::remove_file(path)?;
        }
    }

    download(&jsonschema_url, &schema_path(version))?;
    download(&openapi_url, &openapi_path(version))?;
    Ok(())
}

fn generate_schema(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = schema_path(version);
    let generated_file = format!("{GENERATE_DIR}/schema_gen.rs",);
    let mut schema: RootSchema = serde_json::from_reader(&File::open(path)?)?;

    for schema in schema.definitions.values_mut() {
        cleanup_schema(schema);
    }

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_map_type("::std::collections::BTreeMap")
            .with_derive("::schemars::JsonSchema".to_string()),
    );
    type_space.add_root_schema(schema).unwrap();
    let contents = prettyplease::unparse(
        &syn::parse2::<syn::File>(type_space.to_stream()).unwrap(),
    );
    fs::write(generated_file, contents)?;

    Ok(())
}

//fn generate_openapi(version: &str) -> Result<(), Box<dyn std::error::Error>> {
//    let path = openapi_path(version);
//    let generated_file = format!("{GENERATE_DIR}/openapi_gen.rs",);
//    let openapi: Value = serde_json::from_reader(&File::open(path)?)?;
//
//    let mut output = vec![];
//    for (path, item) in openapi["paths"].as_object().unwrap() {
//        let mut fields = vec![];
//        for parameter in collect_path_schemas(item) {
//            let ptype = get_type(&parameter["schema"]);
//            let name = parameter["name"].as_str().unwrap();
//            let ident = CaseStyle::guess(name).unwrap();
//            let ident = ident.to_camelcase();
//            let ident = if ident == "type" {
//                format_ident!("type_")
//            } else {
//                format_ident!("{}", ident)
//            };
//            fields.push(quote! {
//                pub #ident: #ptype
//            });
//        }
//        let doc = format!(" Path: {}", path);
//        let ident = get_identifier_for_path(path);
//        let ident = format_ident!("{}", ident);
//        output.push(quote! {
//            #[doc = #doc]
//            #[derive(Debug, Clone, ::derive_builder::Builder, ::schemars::JsonSchema, ::serde::Serialize, ::serde::Deserialize)]
//            struct #ident {
//                #(#fields),*
//            }
//        });
//
//        if let Some(get) = item.get("get") {
//            output.push(impl_response("Get", &ident, get));
//        }
//        if let Some(get) = item.get("put") {
//            output.push(impl_response("Put", &ident, get));
//        }
//        if let Some(get) = item.get("post") {
//            output.push(impl_response("Post", &ident, get));
//        }
//        if let Some(get) = item.get("delete") {
//            output.push(impl_response("Delete", &ident, get));
//        }
//    }
//
//    let contents = prettyplease::unparse(&syn::parse2::<syn::File>(
//        quote! { #(#output)* },
//    )?);
//    println!("{}", contents);
//    Ok(())
//}

fn generate(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    generate_schema(version)?;
    //generate_openapi(version)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let version = "26.2.0";

    if opts.update {
        update(&version)?;
    }
    if opts.generate {
        generate(&version)?;
    }
    Ok(())
}
