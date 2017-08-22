extern crate select;

use select::document::Document;
use select::node::Node;

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

fn parse_ruby(node: &Node) -> Option<Fragment> {
    use select::predicate::{Name, Text};

    node.find(Text).next().map(|text| {
        let furigana = node.find(Name("rt")).next().map(|rt| rt.text());

        Fragment {
            text: text.text(),
            furigana,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ruby() {
        use select::predicate::Name;

        let doc = Document::from("<ruby>強<rt>つよ</rt></ruby>");
        let ruby = doc.find(Name("ruby")).next().unwrap();

        assert_eq!(
            parse_ruby(&ruby),
            Some(Fragment {
                text: "強".into(),
                furigana: Some("つよ".into()),
            })
        );
    }
}
