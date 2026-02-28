mod load;
mod patches;

use clap::Parser;
use std::fs;
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
const GENERATE_DIR: &str = root_dir!("keycloak-types2/src");

#[derive(Debug, Parser)]
struct Opts {
    #[clap(long)]
    generate: bool,

    #[clap(long, default_value = "26.2.0")]
    version: String,

    #[clap(long)]
    download: bool,
}

fn openapi_path(version: &str) -> String {
    format!("{SCHEMAS_DIR}/keycloak_openapi-v{version}.json")
}

fn download(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.keycloak.org/docs-api/{version}/rest-api/openapi.json"
    );
    let out_path = format!(
        "{SCHEMAS_DIR}/keycloak_openapi-v{version}.json"
    );
    fs::create_dir_all(SCHEMAS_DIR)?;
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(&url).send()?.error_for_status()?;
    let mut out_file = fs::File::create(&out_path)?;
    std::io::copy(&mut response, &mut out_file)?;
    eprintln!("Downloaded {url} -> {out_path}");
    Ok(())
}

fn generate(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = openapi_path(version);
    let generated_file = format!("{GENERATE_DIR}/schema_gen.rs");

    // 1. Load OpenAPI -> RootSchema
    eprintln!("Loading {path}...");
    let mut schema = load::load_openapi_as_root_schema(&path)?;

    // 2. Patch schema (keycloak bugs)
    patches::patch_schema(&mut schema);

    // 3. Generate Rust types with typify
    eprintln!("Generating types...");
    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_map_type("::std::collections::BTreeMap")
            .with_derive("::schemars::JsonSchema".to_string()),
    );
    type_space.add_root_schema(schema)?;
    let token_stream = type_space.to_stream();

    // 4. Parse and post-process for K8s attributes
    let mut ast: syn::File = syn::parse2(token_stream)?;
    patches::add_k8s_attrs(&mut ast);

    // 5. Format and write
    let contents = prettyplease::unparse(&ast);
    fs::write(&generated_file, &contents)?;
    eprintln!("Wrote {generated_file} ({} bytes)", contents.len());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    if opts.download {
        download(&opts.version)?;
    }
    if opts.generate {
        generate(&opts.version)?;
    }
    if !opts.download && !opts.generate {
        eprintln!("Nothing to do. Use --generate and/or --download.");
    }
    Ok(())
}
