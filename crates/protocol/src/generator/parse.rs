use crate::generator::structs::ApiSpec;
use anyhow::Context;
use std::{fmt::Write, fs, fs::read_dir, path::Path, string::String};

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
            .skip_while(|f| f.trim().starts_with("//"))
            .fold((vec![], String::new()), |mut output, f| {
                if f.trim().starts_with("//") {
                    output
                        .0
                        .push(f.replace("//", "").replace('\n', "").trim().to_string());
                } else {
                    let _ = writeln!(output.1, "{f}");
                }
                output
            });
        let mut api_spec: ApiSpec =
            serde_json::from_str(&content.1).context(format!("Failed to parse {filename}"))?;
        api_spec.api_versions_comment = content.0;
        specs.push(api_spec);
    }
    Ok(specs)
}
