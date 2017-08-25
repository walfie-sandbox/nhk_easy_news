extern crate select;

mod token;

use std::borrow::Cow;
use token::Tokens;

#[derive(Debug, PartialEq, Eq)]
pub struct Image<'a> {
    pub url: String,
    pub caption: Cow<'a, str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Article<'a> {
    pub title: Tokens<'a>,
    pub image: Image<'a>,
    pub video: Option<Cow<'a, str>>,
    pub paragraphs: Vec<Tokens<'a>>,
}
