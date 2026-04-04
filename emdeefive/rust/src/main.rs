use scraper::{Html, Selector};

use clap::Parser;

#[derive(Parser)]
struct Cli {
     #[arg(short, long)]
    target: std::net::SocketAddr,

    #[arg(short, long)]
    proxy: Option<std::net::SocketAddr>,
}

//Parses request bodies for the required element
fn el_parse(body: &str, selector: &str) -> Option<String> {

	let document = Html::parse_document(&body);
    
    let element = Selector::parse(selector).unwrap();

    document
        .select(&element)
        .next()
        .map(|el| el.text().collect::<String>())
}

fn create_hash(hash: &str) -> String {
    format!("{:x}", md5::compute(hash.as_bytes()))
}

fn main() -> Result<(), reqwest::Error> {

	let args = Cli::parse();

    let mut builder = reqwest::blocking::Client::builder()
    .cookie_store(true);

    if let Some(proxy) = args.proxy {
        builder = builder.proxy(reqwest::Proxy::http(format!("http://{}",proxy))?);
    }

    let client = builder.build()?;
    
    let url = format!("http://{}", args.target);

    let body = client.get(&url).send()?.text()?;
    
    let mut hash = None;

    if let Some(text) = el_parse(&body, "h3") {
        println!("Text to hash: {}", text );
        hash = Some(create_hash(&text));
        println!("MD5 Hash: {:?}", hash);
    }

    let Some(hash) = hash else {
    println!("Nothing found to hash");
    return Ok(());
};

    let res = client.post(url)
    .form(&[("hash", hash)])    
    .send();

    let res_body = res?.text()?;

    let flag = el_parse(&res_body, "p");
    println!("Flag: {}", flag.unwrap_or_else(|| "No flag, try again!".to_string()));

    Ok(())
}


