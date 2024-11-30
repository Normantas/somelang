use logos::{Lexer, Logos};
use std::num::{IntErrorKind, ParseIntError};
use thiserror::Error;
use unescape::unescape;

#[derive(Debug, Error, Clone, PartialEq, Eq, Default)]
pub enum LexingError {
    #[default]
    #[error("unknown token")]
    Unknown,
    #[error("failed parsing into a number")]
    ParseIntError(IntErrorKind),
}

impl From<ParseIntError> for LexingError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err.kind().clone())
    }
}

#[derive(Logos, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\r\f]+")]
pub enum Token {
    // Literals
    #[regex("[a-zA-Z0-9_]+", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r#""[^"]*""#, |lex| unescape(&lex.slice().to_string().replace("\"", "")).unwrap())]
    String(String),

    #[regex(r"(0x[0-9a-fA-F]+|0b[01]+|\d+)", priority = 3, callback = |lex| {
		let string = lex.slice();
		if string.starts_with("0x") {
			i64::from_str_radix(&string.replace("0x", ""), 16)
		} else if string.starts_with("0b") {
			i64::from_str_radix(&string.replace("0b", ""), 2)
		} else if string.starts_with("-") {
			Ok(-(string.replace("-", "").parse::<i64>()?))
		} else {
			string.parse()
		}
	})]
    Number(i64),

    
    // Keywords
    #[token("fn")]
    Fn,


    // Symbols
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(",")]
    Comma,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("+=", priority = 5)]
    AddEqual,

    #[token("-=", priority = 5)]
    SubEqual,

    #[token("*=", priority = 5)]
    MulEqual,

    #[token("/=", priority = 5)]
    DivEqual,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[regex(r"//[^\n]*", logos::skip)]
    Comment,

    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    MultiComment,
}

pub fn lex(source: &str) -> Lexer<'_, Token> {
    Token::lexer(source)
}

// Winnow Stream implementation

impl winnow::stream::ContainsToken<Token> for Token {
    #[inline(always)]
    fn contains_token(&self, token: Token) -> bool {
        *self == token
    }
}

impl winnow::stream::ContainsToken<Token> for &'_ [Token] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}

impl<const LEN: usize> winnow::stream::ContainsToken<Token> for &'_ [Token; LEN] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}

impl<const LEN: usize> winnow::stream::ContainsToken<Token> for [Token; LEN] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}
