use scraper::{Html, Selector};
use ureq;

struct Point {
    text: String,
    number: String,
}

fn get_page_content(url: &str) -> Result<String, ureq::Error> {
    let content: String = ureq::get(&url).call()?.into_string()?;
    Ok(content)
}

fn get_div_point(html: &str) -> Point {
    let document = Html::parse_document(html);

    let point_selector = Selector::parse("div.imperavi-body").unwrap();
    let point = document.select(&point_selector);

    // Point contains a number and a text. I want to be able to just print it for now, and then store it on Point struct
    let mut current_point = Point {
        number: String::new(),
        text: String::new(),
    };

    for node in point {
        let number_selector = Selector::parse("h1").unwrap();
        let text_selector = Selector::parse("p").unwrap();

        if let Some(number) = node.select(&number_selector).next() {
            current_point.number = number.text().collect();
        }

        if let Some(text) = node.select(&text_selector).next() {
            current_point.text = text.text().collect();
        }
    }

    current_point
}

fn main() {
    let path = String::from("https://escriva.org/pt-br/camino/999");
    //Getting the page's HTML content
    let html: String = get_page_content(&path).unwrap();
    //Parsing specific div
    let pt_167 = get_div_point(&html);

    println!("{}", pt_167.number);
    println!("{}", pt_167.text);
}
