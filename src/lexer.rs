use regex::Regex;

#[derive(Debug, Clone)]
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

    'src: while !src.is_empty() {
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
