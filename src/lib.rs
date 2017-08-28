extern crate select;

mod token;

use select::document::Document;
use select::node::Node;
use select::predicate;
use std::borrow::Cow;
use token::Tokens;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image<'a> {
    pub url: Cow<'a, str>,
    pub caption: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Article<'a> {
    pub title: Tokens<'a>,
    pub image: Option<Image<'a>>,
    pub video: Option<Cow<'a, str>>,
    pub paragraphs: Vec<Tokens<'a>>,
}

fn parse_article_from_document<'a>(document: &'a Document) -> Option<Article<'a>> {
    document
        .find(predicate::Attr("id", "main"))
        .next()
        .and_then(parse_article_from_node)
}

fn parse_article_from_node<'a>(root: Node<'a>) -> Option<Article<'a>> {
    let title = root.find(predicate::Attr("id", "newstitle"))
        .next()
        .and_then(|n| n.first_child())
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

    let video: Option<Cow<str>> = image_element
        .and_then(|n| n.find(predicate::Attr("class", "playBT")).next())
        .and_then(|n| n.attr("id").map(Into::into));

    let paragraphs: Vec<Tokens> = root.find(predicate::Descendant(
        predicate::Attr("id", "newsarticle"),
        predicate::Name("p"),
    )).map(token::parse_tokens)
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
