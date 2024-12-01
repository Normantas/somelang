use winnow::combinator::{repeat_till, seq};
use winnow::token::one_of;
use winnow::{PResult, Parser};

use crate::lex::Token;

use super::stmt::{parse_stmt, Stmt};

#[derive(Debug, PartialEq)]
pub struct Function {
    name: String,
    body: Vec<Stmt>,
}

pub fn parse_function(input: &mut &[Token]) -> PResult<Function> {
    seq! {Function {
        _: one_of(Token::Fn),
        name: one_of(|t| matches!(t, Token::Ident(_))).map(|t| match t {
            Token::Ident(v) => v,
            _ => unreachable!(),
        }),
        _: one_of(Token::LBrace),
        body: repeat_till(0.., parse_stmt, one_of(Token::RBrace)).map(|x| {
            x.0
        }),
    }}
    .parse_next(input)
}
