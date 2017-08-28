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

    let mut vocab = Vec::new();
    let mut locations = Vec::new();
    let mut names = Vec::new();

    for paragraph in article.paragraphs.iter() {
        for token in paragraph.0.iter() {
            use Token::*;

            match *token {
                Location(ref fragments) => locations.push(fragments),
                Other(ref fragment) => vocab.push(fragment),
                Name(ref fragments) => names.push(fragments),
            }
        }
    }

    if !vocab.is_empty() {
        println!("Vocabulary:");
        vocab.retain(|frag| frag.furigana.is_some());
        vocab.sort_by_key(|frag| &frag.furigana);
        vocab.dedup_by_key(|frag| &frag.text);
        for v in vocab {
            print_fragment(v);
            println!("");
        }

        println!("");
    }

    if !locations.is_empty() {
        // TODO: Sort, dedupe
        println!("Locations:");
        for l in locations {
            print_fragments(l);
            println!("");
        }
        println!("");
    }

    if !names.is_empty() {
        // TODO: Sort, dedupe
        println!("Names:");
        for n in names {
            print_fragments(n);
            println!("");
        }
        println!("");
    }

}
