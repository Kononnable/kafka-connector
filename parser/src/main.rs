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

    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("pre").unwrap();

    let definitions: Vec<String> = fragment
        .select(&selector)
        .skip(6) // example & headers
        .map(|x| x.inner_html().replace("&gt;", ">"))
        .collect();

    let definitions: Vec<_> = definitions
        .iter()
        .map(|x| parse_api_call(&x).unwrap().1)
        .collect();

    let definitions = group_api_calls(definitions);

    let base_path = Path::new("./generated/");
    remove_dir_all(base_path)?;
    DirBuilder::new().recursive(true).create(base_path)?;

    for (key, grouped_call) in definitions.into_iter() {
        let file_name = format!("{}{}.rs", &base_path.display(), to_snake_case(key));
        let path = Path::new(&file_name);
        println!("{}", path.display());

        let content = generate_content(grouped_call);

        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}
