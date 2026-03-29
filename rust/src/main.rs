use scraper::{Html, Selector};

fn main() -> Result<(), reqwest::Error> {

    let client = reqwest::blocking::Client::builder()
    .cookie_store(true)
    .proxy(reqwest::Proxy::http("http://127.0.0.1:8080")?)
    .build()
    .unwrap();
    
    let url = "http://154.57.164.75:31226";

    let body = client.get(url).send()?.text()?;

    println!("{}", &body);

    let document = Html::parse_document(&body);
    
    let h3 = Selector::parse("h3").unwrap();

    let mut hash = None;

    if let Some(element) = document.select(&h3).next() {
        let text = element.text().collect::<String>();
        println!("{}", text );
        hash = Some(format!("{:x}", md5::compute(text.as_bytes())));
        println!("{:?}", hash);
    }

    let res = client.post(url)
    .form(&[("hash", hash.unwrap())])    
    .send();

    println!("{}", res.unwrap().text().unwrap());

    Ok(())
}


