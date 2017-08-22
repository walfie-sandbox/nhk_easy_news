#[macro_use]
extern crate html5ever;

use html5ever::QualName;
use html5ever::rcdom::{self, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

pub struct Image {
    pub url: String,
    pub caption: Option<String>,
}

pub struct Article {
    pub title: Tokens,
    pub image: Image,
    pub video: Option<String>,
    pub paragraphs: Vec<Tokens>,
}

pub struct Tokens(pub Vec<Token>);

#[derive(Debug, PartialEq, Eq)]
pub struct Fragment {
    pub text: String,
    pub furigana: Option<String>,
}

impl<S> From<S> for Fragment
where
    S: Into<String>,
{
    fn from(text: S) -> Fragment {
        Fragment {
            text: text.into(),
            furigana: None,
        }
    }
}

pub enum Token {
    Location(Vec<Fragment>),
    Name(Vec<Fragment>),
    Other(Fragment),
}

/*
fn parse_token(node: rcdom::Handle) -> Option<Token> {
    match node.data {
        NodeData::Element { ref name, ref attrs } => {
            match name.local {
                local_name!("ruby") => {
                    node.children
                }
                local_name!("span") => {

                }
            }
        }
        NodeData::Text { ref contents } =>
            return Some(Token::Other(Fragment::from(contents)));
        _ => (),
    }
}
*/

fn parse_ruby(nodes: &[rcdom::Handle]) -> Option<Fragment> {
    let mut out_text = None;
    let mut furigana = None;

    for node in nodes {
        match node.data {
            NodeData::Text { ref contents, .. } => {
                out_text = Some(contents.borrow().to_string());
            }
            NodeData::Element { ref name, .. } if name.local == local_name!("rt") => {
                if let Some(child) = node.children.borrow().first() {
                    if let NodeData::Text { ref contents, .. } = child.data {
                        furigana = Some(contents.borrow().to_string());
                    }
                }
            }
            NodeData::Element { ref name, .. } => {
                println!("{:?}", name);
            }
            _ => {}
        }
    }

    println!("out_text: {:?}", out_text);
    println!("furigana: {:?}", furigana);

    out_text.map(|text| Fragment { text, furigana })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ruby() {
        // Fails
        let node = html5ever::driver::parse_fragment(
            RcDom::default(),
            Default::default(),
            QualName::new(None, ns!(html), local_name!("body")),
            Vec::new(),
        ).one("<ruby>強<rt>つよ</rt></ruby>");

        assert_eq!(
            parse_ruby(&node.document.children.borrow()),
            Some(Fragment {
                text: "強".into(),
                furigana: Some("つよ".into()),
            })
        );
    }
}
