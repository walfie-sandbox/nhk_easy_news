extern crate select;

mod token;

use select::document::Document;
use select::node::Node;
use select::predicate;
pub use token::{Fragment, Token, Tokens};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    pub url: String,
    pub caption: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Article {
    pub title: Tokens,
    pub image: Option<Image>,
    pub video: Option<String>,
    pub paragraphs: Vec<Tokens>,
}

pub fn parse_article(html_string: &str) -> Option<Article> {
    Document::from(html_string)
        .find(predicate::Attr("id", "main"))
        .next()
        .and_then(parse_article_from_node)
}

fn parse_article_from_node<'a>(root: Node<'a>) -> Option<Article> {
    let title = root.find(predicate::Attr("id", "newstitle"))
        .next()
        .and_then(|n| n.find(predicate::Name("h2")).next())
        .map(token::parse_tokens);

    let image_element = root.find(predicate::Attr("id", "mainimage")).next();

    let image = image_element
        .and_then(|n| n.find(predicate::Name("img")).next())
        .and_then(|n| {
            n.attr("src").map(|url| {
                Image {
                    url: url.into(),
                    caption: n.attr("alt").map(Into::into),
                }
            })
        });

    let video: Option<String> = image_element
        .and_then(|n| n.find(predicate::Attr("class", "playBT")).next())
        .and_then(|n| n.attr("id").map(Into::into));

    let paragraphs: Vec<Tokens> = root.find(predicate::Descendant(
        predicate::Attr("id", "newsarticle"),
        predicate::Name("p"),
    )).map(token::parse_tokens)
        .filter(|tokens| !tokens.0.is_empty())
        .collect();

    title.map(|t| {
        Article {
            title: t,
            image,
            video,
            paragraphs,
        }
    })
}
