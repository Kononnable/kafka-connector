use std::{
    fs::{read_to_string, remove_dir_all, DirBuilder, File},
    io::Write,
    path::Path,
};

use parser::{
    generator::generate_content, parser::parse_api_call, transformer_step1::group_api_calls,
    transformer_step2::transform, utils::to_snake_case,
};
use regex::Regex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let body = reqwest::get("https://kafka.apache.org/protocol")
    //     .await?
    //     .text()
    // .await?;
    let body = read_to_string("./parser/protocol.html")?;

    let regex = Regex::new(r"(?m)<pre>([^<]+)</pre>").unwrap();
    let capture_groups = regex.captures_iter(&body);

    let api_definitions: Vec<String> = capture_groups
        .skip(5) // example & headers
        .map(|x| x[1].replace("&gt;", ">"))
        .collect();

    let parsed_definitions: Vec<_> = api_definitions
        .iter()
        .map(|definition| parse_api_call(definition).unwrap().1)
        .collect();

    let transformed_definitions = group_api_calls(parsed_definitions);

    let base_path = Path::new("./generated/");
    remove_dir_all(base_path)?;
    DirBuilder::new().recursive(true).create(base_path)?;

    for (key, grouped_call) in transformed_definitions.into_iter() {
        let file_name = format!("{}{}.rs", &base_path.display(), to_snake_case(key));
        let path = Path::new(&file_name);
        println!("{}", path.display());

        let transformed = transform(key, grouped_call);
        let content = generate_content(transformed);

        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}
