// build.rs
use std::env;
use std::path::Path;
use std::process::Command;
fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/gresource.xml",
        "app.gresource",
    );

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let schema_dir = Path::new(&manifest_dir).join("resources");

    if schema_dir.exists() {
        validate_schemas(&schema_dir);
        compile_schemas(&schema_dir);

        println!("cargo:rustc-env=GSETTINGS_SCHEMA_DIR={}", schema_dir.display());
        println!("cargo:rerun-if-changed=schemas/");
    }
}

fn validate_schemas(schema_dir: &Path) {
    let status = Command::new("glib-compile-schemas")
        .arg("--dry-run")
        .arg(schema_dir)
        .status()
        .expect("Failed to validate schemas");

    if !status.success() {
        panic!("GSchemas contain errors");
    }
}

fn compile_schemas(schema_dir: &Path) {
    let status = Command::new("glib-compile-schemas")
        .arg(schema_dir)
        .status()
        .expect("Failed to compile schemas");

    if !status.success() {
        panic!("Failed to compile gsettings schemas");
    }
}
