// build.rs
use anyhow;
use std::env;
use std::path::Path;
use std::process::Command;
fn main() -> Result<(), anyhow::Error> {
    // 扫描src目录下的所有blp文件
    let src_dir = Path::new("src");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let schema_dir = Path::new(&manifest_dir).join("resources");
    let vec = scan_blp_files(&src_dir);

    // 编译blp文件到resources目录
    vec.iter().for_each(|blp_file| {
        println!("cargo:rerun-if-changed={}", blp_file.display());
        compile_blp_to_xml(blp_file, &schema_dir).expect("Failed to compile blp file");
    });

    glib_build_tools::compile_resources(&["resources"], "resources/gresource.xml", "app.gresource");

    if env::var("DOCS_RS").is_ok() {}

    validate_schemas(&schema_dir);
    compile_schemas(&schema_dir);

    println!(
        "cargo:rustc-env=GSETTINGS_SCHEMA_DIR={}",
        schema_dir.display()
    );
    println!("cargo:rerun-if-changed=schemas/");

    Ok(())
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

fn scan_blp_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut blp_files = Vec::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                blp_files.extend(scan_blp_files(&path));
            } else if let Some(extension) = path.extension() {
                if extension == "blp" {
                    blp_files.push(path);
                }
            }
        }
    }

    blp_files
}

// 新增函数：调用blueprint编译器将blp文件编译为xml文件
fn compile_blp_to_xml(blp_file: &Path, output_dir: &Path) -> Result<(), anyhow::Error> {
    let xml_file = output_dir.join(
        match match blp_file.file_name() {
            Some(x) => x,
            None => todo!(),
        }
        .to_str()
        {
            Some(x) => x,
            None => todo!(),
        }
        .replace(".blp", ".ui"),
    );

    // 确保resources目录存在
    std::fs::create_dir_all("resources")?;

    let output = Command::new("blueprint-compiler")
        .arg("compile")
        .arg("--output")
        .arg(&xml_file)
        .arg(blp_file)
        .output()
        .expect("Failed to execute blueprint-compiler");

    if !output.status.success() {
        panic!(
            "Failed to compile {}: {}",
            blp_file.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
