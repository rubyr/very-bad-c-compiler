use std::collections::VecDeque;

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Constant,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,

    KInt,
    KVoid,
    KRet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: usize,
    pub index: usize,
    pub token_type: TokenType,
    pub substring: String,
}

use TokenType::*;

pub type Tokens = VecDeque<Token>;

pub fn lex(src: String) -> Tokens {
    let mut tokens = VecDeque::new();

    macro_rules! rgx {
        ($s: expr) => {
            Regex::new($s).expect(&format!("failed to create regex \"{}\"", $s))
        }
    }

    let line_comments = rgx!(r"\/\/.*");
    let block_comments = rgx!(r"\/\*.*?\*\/");

    let src = block_comments.replace_all(&src, "").to_string();
    let src = line_comments.replace_all(&src, "");
    let src = src.trim();
    let mut pos = 0;

    let matchers = [
        (r"^int\b", KInt),
        (r"^void\b", KVoid),
        (r"^return\b", KRet),
        (r"^[a-zA-Z_]\w*\b", Identifier),
        (r"^[0-9]+\b", Constant),
        (r"^\(", LParen),
        (r"^\)", RParen),
        (r"^\{", LBrace),
        (r"^\}", RBrace),
        (r"^;", Semi),
    ]
    .map(|(re, t)| (rgx!(re), t));

    let whitespace = rgx!(r"^\s+");

    /* 'src: */
    while pos < src.len() {
        let mut found = false;
        'hay: for (m, t) in &matchers {
            if let Some(m) = m.find(&src[pos..]) {
                found = true;
                let match_end = m.end() + pos;
                dbg!(match_end);

                pos = whitespace
                    .find(&src[match_end..])
                    .map(|m| m.end() + match_end)
                    .unwrap_or(match_end);

                let lines = src[..pos].lines();
                let line = lines.clone().count();
                dbg!(pos, &lines.clone().collect::<Vec<_>>());
                let index = lines.last().map(|s| s.len()).unwrap_or(0);

                tokens.push_back(Token {
                    token_type: t.clone(),
                    substring: m.as_str().to_string(),
                    line,
                    index,
                });

                break 'hay;
            }
        }

        if !found {
            // throw error
            // break 'src;

            panic!("error while lexing: {:?}\nsrc: {:?}\npos: {:?}", tokens, &src[pos..], pos);
        }
    }

    tokens
}

mod test {
    use std::collections::VecDeque;

    use super::Tokens;
    #[allow(unused_imports)]
    use super::{lex, Token, TokenType::*};

    #[allow(dead_code)] // this is just a helper fn
    fn ret_n(n: i32) -> VecDeque<super::Token> {
        VecDeque::from(vec![
            Token {
                token_type: KInt,
                line: 1,
                index: 0,
                substring: "int".to_string(),
            },
            Token {
                token_type: Identifier,
                line: 1,
                index: 3,
                substring: "main".to_string(),
            },
            Token {
                token_type: LParen,
                line: 1,
                index: 7,
                substring: "(".to_string(),
            },
            Token {
                token_type: KVoid,
                line: 1,
                index: 8,
                substring: "void".to_string(),
            },
            Token {
                token_type: RParen,
                line: 1,
                index: 12,
                substring: ")".to_string(),
            },
            Token {
                token_type: LBrace,
                line: 1,
                index: 13,
                substring: "{".to_string(),
            },
            Token {
                token_type: KRet,
                line: 2,
                index: 14,
                substring: "return".to_string(),
            },
            Token {
                token_type: Constant,
                line: 2,
                index: 20,
                substring: n.to_string(),
            },
            Token {
                token_type: Semi,
                line: 2,
                index: 20 + n.to_string().len(),
                substring: ";".to_string(),
            },
            Token {
                token_type: RBrace,
                line: 3,
                index: 21 + n.to_string().len(),
                substring: "}".to_string(),
            },
        ])
    }

    fn ignore_lines(t: Tokens) -> Tokens {
        t.iter().map(|t| Token { line: 0, index: 0, ..t.clone() }).collect()
    }

    #[test]
    fn lexes_nothing() {
        assert_eq!(lex(String::new()), vec![]);
    }

    #[test]
    fn lexes_many_tokens() {
        let src = r"
        int main(void) {
            return 0;
        }";
        assert_eq!(ignore_lines(lex(src.to_string())), ignore_lines(ret_n(0)));
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
        assert_eq!(ignore_lines(lex(src.to_string())), ignore_lines(ret_n(0)));
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
        assert_eq!(ignore_lines(lex(src.to_string())), ignore_lines(ret_n(0)));
    }
}
