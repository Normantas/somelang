use winnow::combinator::{repeat_till, separated, seq};
use winnow::token::one_of;
use winnow::{PResult, Parser};

use crate::lex::Token;

use super::stmt::{parse_stmt, Stmt};

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionArg {
    pub name: String,
    pub arg_type: String,
}

pub fn parse_function(input: &mut &[Token]) -> PResult<Function> {
    seq! {Function {
        _: one_of(Token::Fn),
        name: one_of(|t| matches!(t, Token::Ident(_))).map(|t| match t {
            Token::Ident(v) => v,
            _ => unreachable!(),
        }),
        _: one_of(Token::LParen),
        args: separated(0.., parse_function_arg, one_of(Token::Comma)),
        _: one_of(Token::RParen),
        _: one_of(Token::LBrace),
        body: repeat_till(0.., parse_stmt, one_of(Token::RBrace)).map(|x| {
            x.0
        }),
    }}
    .parse_next(input)
}

pub fn parse_function_arg(input: &mut &[Token]) -> PResult<FunctionArg> {
    seq! {FunctionArg {
        name: one_of(|t| matches!(t, Token::Ident(_))).map(|t| match t {
            Token::Ident(v) => v,
            _ => unreachable!(),
        }),
        _: one_of(Token::Colon),
        arg_type: one_of(|t| matches!(t, Token::Ident(_))).map(|t| match t {
            Token::Ident(v) => v,
            _ => unreachable!(),
        }),
    }}
    .parse_next(input)
}
