use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier { value: String },
    Constant { value: i32 },
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,

    KInt,
    KVoid,
    KRet,
}

use Token::*;

pub fn lex(src: String) -> Vec<Token> {
    let mut tokens = vec![];

    let line_comments = Regex::new(r"\/\/.*").expect("failed to create regex");
    let block_comments = Regex::new(r"\/\*.*?\*\/").expect("failed to create regex");

    let src = block_comments.replace_all(&src, "").to_string();
    let src = line_comments.replace_all(&src, "");
    let mut src = src.trim();

    let matchers = [
        (r"^int\b", KInt),
        (r"^void\b", KVoid),
        (r"^return\b", KRet),
        (
            r"^[a-zA-Z_]\w*\b",
            Identifier {
                value: String::new(),
            },
        ),
        (r"^[0-9]+\b", Constant { value: 0 }),
        (r"^\(", LParen),
        (r"^\)", RParen),
        (r"^\{", LBrace),
        (r"^\}", RBrace),
        (r"^;", Semi),
    ]
    .map(|(re, t)| (Regex::new(re).expect("failed to create regex"), t));

    /* 'src: */
    while !src.is_empty() {
        let mut found = false;
        'hay: for (m, t) in &matchers {
            if let Some(m) = m.find(src) {
                found = true;

                src = &src[m.len()..];

                match t {
                    Token::Identifier { value: _ } => tokens.push(Identifier {
                        value: m.as_str().to_string(),
                    }),
                    Token::Constant { value: _ } => tokens.push(Constant {
                        value: m.as_str().parse().expect("could not parse constant"),
                    }),
                    _ => tokens.push(t.clone()),
                }

                break 'hay;
            }
        }

        if !found {
            // throw error
            // break 'src;

            panic!("error while lexing: {:?}", tokens);
        }

        src = src.trim_start();
    }

    tokens
}

mod test {
    #[allow(unused_imports)]
    use super::{lex, Token::*};

    #[allow(dead_code)] // this is just a helper fn
    fn ret_n(n: i32) -> Vec<super::Token> {
        vec![
            KInt,
            Identifier {
                value: "main".to_string(),
            },
            LParen,
            KVoid,
            RParen,
            LBrace,
            KRet,
            Constant { value: n },
            Semi,
            RBrace,
        ]
    }

    #[test]
    fn lexes_nothing() {
        assert_eq!(lex(String::new()), vec![]);
    }

    #[test]
    fn lexes_tokens() {
        let tokens = [
            ("int", KInt),
            ("void", KVoid),
            ("return", KRet),
            (
                "i",
                Identifier {
                    value: "i".to_string(),
                },
            ),
            (
                "foo",
                Identifier {
                    value: "foo".to_string(),
                },
            ),
            ("0", Constant { value: 0 }),
            ("1000", Constant { value: 1000 }),
            ("(", LParen),
            (")", RParen),
            ("{", LBrace),
            ("}", RBrace),
            (";", Semi),
        ];
        for (src, token) in tokens {
            assert_eq!(lex(src.to_string()), vec![token]);
        }
    }

    #[test]
    fn lexes_many_tokens() {
        let src = r"
        int main(void) {
            return 0;
        }
        ";
        assert_eq!(lex(src.to_string()), ret_n(0));
    }

    #[test]
    fn line_comments() {
        let src = r"
        // this shouldn't trip it up!
        int main(void) {
            // this shouldn't trip it up!
            return 0;
        }
        // this shouldn't trip it up!
        ";
        assert_eq!(lex(src.to_string()), ret_n(0));
    }

    #[test]
    fn block_comments() {
        let src = r"
        /* /* it doesn't support nested comments :( */
        int /* it should ignore this */ main(void) {
            return 0; /* and this */
        }
        /* ham burger */
        ";
        assert_eq!(lex(src.to_string()), ret_n(0));
    }
}
