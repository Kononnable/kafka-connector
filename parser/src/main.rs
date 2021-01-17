use parser::{generator::generate_content, parser::parse_api_call, transformer::group_api_calls};
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
        .map(|x| {
          println!("{}",x);
          parse_api_call(&x).unwrap().1
        })
        .collect();
    
    let definitions = group_api_calls(definitions);

    for (key,grouped_call) in definitions.into_iter() {
        let content = generate_content(grouped_call);
        println!("Name: {} Definitions: {}", key,content);
    }

    
    Ok(())
}
