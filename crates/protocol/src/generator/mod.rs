use crate::generator::structs::ApiSpec;
use anyhow::{bail, Context};
use convert_case::{Case, Casing};
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

mod codegen;
mod parse;
mod structs;

#[test]
fn generates_structures() -> anyhow::Result<()> {
    use expect_test::expect_file;
    const FILE_PATH_PREFIX: &str = "/src/generated/";
    let specs = parse::get_api_specs()?;
    // .into_iter()
    // .filter(|x| x.name == "DescribeAclsResponse")
    // .collect::<Vec<_>>();
    let mut generated = specs
        .iter()
        .map(codegen::generate_source)
        .collect::<Vec<_>>();
    generated.push(generate_mod_file(&specs));

    let formatted = generated
        .into_iter()
        .map(|(filename, content)| {
            let formatted = format_code(content)
                .with_context(|| format!("Formatting generated file {filename}.rs failed"))?;
            Ok((filename, formatted))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    for file in formatted {
        let file_name = format!(
            "{}{}{}.rs",
            env::current_dir().unwrap().to_str().unwrap(),
            FILE_PATH_PREFIX,
            file.0
        );

        let expected = expect_file![file_name];
        expected.assert_eq(&file.1);
    }
    Ok(())
}

fn generate_mod_file(specs: &Vec<ApiSpec>) -> (String, String) {
    let mut content = "".to_owned();
    for spec in specs {
        content.push_str(&format!("pub mod {};\n", spec.name.to_case(Case::Snake)))
    }
    ("mod".to_owned(), content)
}

fn format_code(generated: String) -> anyhow::Result<String> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(generated.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().expect("Failed to read stdout");
    let formatted = String::from_utf8(output.stdout).unwrap();
    if formatted.is_empty() {
        let mut source = "".to_owned();
        for (no, line) in generated.lines().enumerate() {
            source.push_str(&format!("{:3}| {}\n", no + 1, line));
        }
        bail!("Failed to format generated code\n{source}");
    }
    Ok(formatted)
}
