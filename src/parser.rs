use crate::error::{CompilerError, ERRORS};
use crate::errors;
use crate::lexer::{Token, TokenType, Tokens};

#[derive(Debug)]
pub enum Program {
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    ret_type: Token,
    ident: Token,
    statement: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(Token),
}

#[derive(Debug)]
struct ParserError {
    line: usize,
    index: usize,
    string: String,
}

impl CompilerError for ParserError {
    fn line(&self) -> usize {
        self.line
    }

    fn index(&self) -> usize {
        self.index
    }

    fn to_string(&self) -> String {
        self.string.clone()
    }
}

pub fn parse_program(tokens: &mut Tokens) -> Option<Vec<Program>> {
    let mut p = vec![];
    while !tokens.is_empty() {
        p.push(Program::Function(parse_fn(tokens)?));
    }
    Some(p)
}

fn parse_fn(tokens: &mut Tokens) -> Option<Function> {
    let ret_type = expect_type(tokens)?;
    let ident = tokens.pop_front()?;
    if ident.token_type != TokenType::Identifier {
        errors!().push(Box::new(ParserError {
            line: ident.line,
            index: ident.index,
            string: format!(
                "Expected {:?}, got {:?}",
                TokenType::Identifier,
                ident.token_type
            ),
        }));
        return None;
    }
    expect(TokenType::LParen, tokens)?;
    expect(TokenType::KVoid, tokens)?;
    expect(TokenType::RParen, tokens)?;
    expect(TokenType::LBrace, tokens)?;
    let statement = parse_statement(tokens)?;
    expect(TokenType::RBrace, tokens)?;
    Some(Function {
        ret_type,
        ident,
        statement,
    })
}

fn parse_statement(tokens: &mut Tokens) -> Option<Statement> {
    expect(TokenType::KRet, tokens)?;
    let ret_val = parse_exp(tokens)?;
    expect(TokenType::Semi, tokens)?;
    Some(Statement::Return(ret_val))
}

fn parse_exp(tokens: &mut Tokens) -> Option<Expression> {
    let int = expect(TokenType::Constant, tokens)?;
    Some(Expression::Constant(int))
}

fn inspect(tokens: &Tokens) -> Option<&Token> {
    tokens.iter().next()
}

fn expect(expected: TokenType, tokens: &mut Tokens) -> Option<Token> {
    let actual = tokens.pop_front();
    if actual == None {
        errors!().push(Box::new(ParserError {
            line: 0,
            index: 0,
            string: "Unexpected end of input".into(),
        }));
        return None;
    }
    let actual = actual.unwrap();
    if actual.token_type != expected {
        errors!().push(Box::new(ParserError {
            line: actual.line,
            index: actual.index,
            string: format!("Expected {:?}, got {}", expected, actual.to_string()),
        }));
        return None;
    }
    Some(actual)
}

fn expect_type(tokens: &mut Tokens) -> Option<Token> {
    let actual = tokens.pop_front();
    if actual == None {
        errors!().push(Box::new(ParserError {
            line: 0,
            index: 0,
            string: "Unexpected end of input".into(),
        }));
        return None;
    }
    let actual = actual.unwrap();
    match actual.token_type {
        TokenType::KInt => Some(actual),
        _ => {
            errors!().push(Box::new(ParserError {
                line: actual.line,
                index: actual.index,
                string: format!("Expected type, got {}", actual.to_string()),
            }));
            None
        }
    }
}
