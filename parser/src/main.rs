use parser::parser::parse_api_call;
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
    for definition in definitions {
        println!("{:#?}", definition);
    }

    Ok(())
}
