extern crate select;

use select::document::Document;
use select::node::Node;

#[derive(Debug, PartialEq, Eq)]
pub struct Image {
    pub url: String,
    pub caption: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Article {
    pub title: Tokens,
    pub image: Image,
    pub video: Option<String>,
    pub paragraphs: Vec<Tokens>,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Location(Vec<Fragment>),
    Name(Vec<Fragment>),
    Other(Fragment),
}

fn parse_token(node: &Node) -> Option<Token> {
    match node.name() {
        Some("span") => {
            let fragments = parse_fragments(node.children());

            match node.attr("class") {
                Some("colorL") => Some(Token::Location(fragments)),
                Some("locationL") => Some(Token::Name(fragments)),
                _ => None,
            }
        }
        Some("a") => node.first_child().and_then(|n| parse_token(&n)),
        _ => parse_fragment(node).map(Token::Other),
    }
}

fn parse_fragment(node: &Node) -> Option<Fragment> {
    match node.name() {
        None => node.as_text().map(Fragment::from),
        Some("ruby") => parse_ruby(node),
        _ => None,
    }
}


fn parse_fragments<'a, N>(nodes: N) -> Vec<Fragment>
where
    N: Iterator<Item = Node<'a>>,
{
    nodes.filter_map(|n| parse_fragment(&n)).collect()
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

    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    #[test]
    fn tokens() {
        use select::predicate::Name;

        let html_string = r#"
        <div>
            <ruby>
                今
                <rt>いま</rt>
            </ruby>
            、
            <span class='colorL'>スイス</span>
            にある
            <a href='javascript:void(0)' class='dicWin' id='id-0000'>
                <ruby>
                    <span class="under">国連</span>
                    <rt>こくれん</rt>
                </ruby>
            </a>
            の
            <span class='colorC'>
                ヨーロッパ
                <ruby>
                    本部
                    <rt>ほんぶ</rt>
                </ruby>
            </span>
            で、
        </div>
        "#
            .replace("    ", "")
            .replace("\n", "");

        let doc = Document::from(html_string.as_ref());
        let contents = doc.find(Name("div")).next().unwrap();

        let nodes = contents
            .children()
            .filter_map(|node| parse_token(&node))
            .collect::<Vec<_>>();
        println!("{:?}", nodes);

        let expected = [
            Token::Other(Fragment {
                text: "今".into(),
                furigana: Some("いま".into()),
            }),
            Token::Other(Fragment {
                text: "、".into(),
                furigana: None,
            }),
            Token::Location(vec![
                Fragment {
                    text: "スイス".into(),
                    furigana: None,
                },
            ]),
            Token::Other(Fragment {
                text: "にある".into(),
                furigana: None,
            }),
            Token::Other(Fragment {
                text: "国連".into(),
                furigana: Some("こくれん".into()),
            }),
            Token::Other(Fragment {
                text: "の".into(),
                furigana: None,
            }),
            Token::Other(Fragment {
                text: "で、".into(),
                furigana: None,
            }),
        ];

        assert_eq!(nodes, expected);
    }
}
