use select::document::Document;
use select::node::Node;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tokens(pub Vec<Token>);

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Location(Vec<Fragment>),
    Name(Vec<Fragment>),
    Other(Fragment),
}

pub(crate) fn parse_tokens<'a>(node: Node<'a>) -> Tokens {
    Tokens(
        node.children()
            .filter_map(|node| parse_token(node))
            .collect::<Vec<_>>(),
    )
}

fn parse_token<'a>(node: Node<'a>) -> Option<Token> {
    match node.name() {
        Some("span") => {
            let fragments = parse_fragments(node.children());

            match node.attr("class") {
                Some("colorL") => Some(Token::Location(fragments)),
                Some("colorN") => Some(Token::Name(fragments)),
                _ => None,
            }
        }
        Some("a") => node.first_child().and_then(|n| parse_token(n)),
        _ => parse_fragment(node).map(Token::Other),
    }
}

fn parse_fragment<'a>(node: Node<'a>) -> Option<Fragment> {
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
    nodes.filter_map(|n| parse_fragment(n)).collect()
}

fn parse_ruby<'a>(node: Node<'a>) -> Option<Fragment> {
    use predicate::{Name, Text};

    node.find(Text).next().map(|text| {
        let furigana = node.find(Name("rt")).next().map(|rt| rt.text().into());

        Fragment {
            text: text.text().into(),
            furigana,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ruby() {
        let doc = Document::from("<ruby>強<rt>つよ</rt></ruby>");
        let ruby = doc.find(predicate::Name("ruby")).next().unwrap();

        assert_eq!(
            parse_ruby(ruby),
            Some(Fragment {
                text: "強".into(),
                furigana: Some("つよ".into()),
            })
        );
    }

    #[test]
    fn tokens() {
        let html_string = r#"<div>
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
        let contents = doc.find(predicate::Name("div")).next().unwrap();

        let expected = vec![
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

        assert_eq!(parse_tokens(contents), Tokens(expected));
    }
}
