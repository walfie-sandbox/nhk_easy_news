extern crate nhk_easy_news;

use nhk_easy_news::{Fragment, Token};
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let article = nhk_easy_news::parse_article(buffer.as_str()).expect("failed to parse article");

    println!("{}\n", article.title);

    for paragraph in article.paragraphs.iter() {
        println!("{}\n", paragraph);
    }

    fn print_fragment(fragment: &Fragment) {
        print!("{}", fragment.text);
        if let Some(ref furigana) = fragment.furigana {
            print!("({})", furigana);
        }
    }

    fn print_fragments(fragments: &[Fragment]) {
        for f in fragments {
            print_fragment(&f)
        }
    }

    for paragraph in article.paragraphs.iter() {
        for token in paragraph.0.iter() {
            use Token::*;

            match *token {
                Location(ref fragments) => {
                    print!("Location: ");
                    print_fragments(fragments);
                    println!("");
                }
                Other(ref fragment) => {
                    if fragment.furigana.is_some() {
                        print!("Vocabulary: ");
                        print_fragment(fragment);
                        println!("");
                    }
                }
                Name(ref fragments) => {
                    print!("Name: ");
                    print_fragments(fragments);
                    println!("");
                }
            }
        }
    }
}
