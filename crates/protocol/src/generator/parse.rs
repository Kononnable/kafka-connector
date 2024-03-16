use crate::generator::structs::ApiSpec;
use anyhow::Context;
use std::fmt::Write;
use std::fs;
use std::fs::read_dir;
use std::path::Path;

pub fn get_api_specs() -> anyhow::Result<Vec<ApiSpec>> {
    let mut message_specs_path = Path::new(env!("CARGO_MANIFEST_DIR")).to_owned();
    message_specs_path.push(Path::new("messages"));

    let mut specs: Vec<ApiSpec> = vec![];
    for entry in read_dir(message_specs_path)? {
        let path = entry?.path();
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        if path.ends_with("README.md") {
            continue;
        }
        let content = fs::read_to_string(path)
            .unwrap()
            .lines()
            .filter(|f| !f.trim().starts_with("//"))
            .fold(String::new(), |mut output, f| {
                let _ = writeln!(output, "{f}");
                output
            });
        specs.push(serde_json::from_str(&content).context(format!("Failed to parse {filename}"))?);
    }
    Ok(specs)
}
