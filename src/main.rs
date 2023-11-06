use scraper::{Html, Selector};
use ureq;

const BASE_URL: &str = "https://escriva.org/pt-br/";

static CONTROL: &[(&str, u32)] = &[("camino", 999), ("surco", 1000), ("forja", 1055)];

struct Point {
    book: String,
    text: String,
    number: u32,
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
        book: String::new(),
        number: 0,
        text: String::new(),
    };

    for node in point {
        let number_selector = Selector::parse("h1").unwrap();
        let text_selector = Selector::parse("p").unwrap();

        if let Some(number) = node.select(&number_selector).next() {
            current_point.number = number.text().collect::<String>().parse().unwrap();
        }

        if let Some(text) = node.select(&text_selector).next() {
            current_point.text = text.text().collect();
        }
    }

    current_point
}

fn main() {
    let mut point_vec = Vec::<Point>::new();

    for &(book, max_point) in CONTROL {
        println!("{book} {max_point}");
        for point in 1..=max_point {
            // Build URL to specified book and point
            let url = format!("{BASE_URL}/{book}/{point}");

            // Obtain HTML page content
            let html = get_page_content(&url).unwrap();

            //Parsing specific div
            let mut point = get_div_point(&html);

            point.book = (&book).to_string();

            println!("{}", point.book);
            println!("{}", point.number);
            println!("{}", point.text);

            point_vec.push(point);
        }
    }

    // //debug
    // for point in point_vec {
    //     println!("Book: {}", point.book);
    //     println!("Number: {}", point.number);
    //     println!("Text: {}", point.text);
    //     println!("---");
    // }
}
