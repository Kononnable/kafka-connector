use std::{
    fs::{remove_dir_all, DirBuilder, File},
    io::Write,
    path::Path,
};

use parser::{
    generator::generate_content, parser::parse_api_call, transformer::group_api_calls,
    utils::to_snake_case,
};
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let body = reqwest::get("https://kafka.apache.org/protocol")
        .await?
        .text()
        .await?;

    let selector = Selector::parse("pre").unwrap();
    let html_fragment = Html::parse_fragment(&body);

    let api_definitions: Vec<String> = html_fragment
        .select(&selector)
        .skip(6) // example & headers
        .map(|x| x.inner_html().replace("&gt;", ">"))
        .collect();

    let parsed_definitions: Vec<_> = api_definitions
        .iter()
        .map(|definition| parse_api_call(&definition).unwrap().1)
        .collect();

    let transformed_definitions = group_api_calls(parsed_definitions);

    let base_path = Path::new("./generated/");
    remove_dir_all(base_path)?;
    DirBuilder::new().recursive(true).create(base_path)?;

    for (key, grouped_call) in transformed_definitions.into_iter() {
        let file_name = format!("{}{}.rs", &base_path.display(), to_snake_case(key));
        let path = Path::new(&file_name);
        println!("{}", path.display());

        let content = generate_content(grouped_call);

        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}
