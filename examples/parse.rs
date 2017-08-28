extern crate nhk_easy_news;

use std::io::{self, Read};


fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let article = nhk_easy_news::parse_article(buffer.as_str());

    println!("{:#?}", article);
}
